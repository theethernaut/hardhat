use std::{fmt::Debug, marker::PhantomData, ops::Range};

use edr_eth::{Address, Bytes, B256, U256};
use revm::{
    db::DatabaseComponentError,
    interpreter::{CallInputs, CreateInputs, Interpreter, InterpreterResult},
    EvmContext, Inspector,
};

use crate::{
    evm::SyncInspector,
    trace::{Trace, TraceCollector},
};

// TODO: Improve this design by introducing a InspectorMut trait

/// Inspector that allows two inspectors to operate side-by-side. The immutable
/// inspector runs first, followed by the mutable inspector. To ensure both
/// inspectors observe a valid state, you have to ensure that only the mutable
/// inspector modifies state. The returned values are solely determined by the
/// mutable inspector.
#[derive(Debug)]
pub struct DualInspector<A, B, DatabaseErrorT>
where
    A: Inspector<DatabaseErrorT>,
    B: Inspector<DatabaseErrorT>,
{
    immutable: A,
    mutable: B,
    phantom: PhantomData<DatabaseErrorT>,
}

impl<A, B, DatabaseErrorT> DualInspector<A, B, DatabaseErrorT>
where
    A: Inspector<DatabaseErrorT>,
    B: Inspector<DatabaseErrorT>,
{
    /// Constructs a `DualInspector` from the provided inspectors.
    pub fn new(immutable: A, mutable: B) -> Self {
        Self {
            immutable,
            mutable,
            phantom: PhantomData,
        }
    }

    /// Returns the two inspectors wrapped by the `DualInspector`.
    pub fn into_parts(self) -> (A, B) {
        (self.immutable, self.mutable)
    }
}

impl<A, B, DatabaseError> Inspector<DatabaseError> for DualInspector<A, B, DatabaseError>
where
    A: Inspector<DatabaseError>,
    B: Inspector<DatabaseError>,
{
    fn initialize_interp(
        &mut self,
        interp: &mut Interpreter,
        context: &mut EvmContext<'_, DatabaseError>,
    ) {
        self.immutable.initialize_interp(interp, context);
        self.mutable.initialize_interp(interp, context);
    }

    fn step(&mut self, interp: &mut Interpreter, context: &mut EvmContext<'_, DatabaseError>) {
        self.immutable.step(interp, context);
        self.mutable.step(interp, context);
    }

    fn log(
        &mut self,
        context: &mut EvmContext<'_, DatabaseError>,
        address: &Address,
        topics: &[B256],
        data: &Bytes,
    ) {
        self.immutable.log(context, address, topics, data);
        self.mutable.log(context, address, topics, data);
    }

    fn step_end(&mut self, interp: &mut Interpreter, data: &mut EvmContext<'_, DatabaseError>) {
        self.immutable.step_end(interp, data);
        self.mutable.step_end(interp, data);
    }

    fn call(
        &mut self,
        data: &mut EvmContext<'_, DatabaseError>,
        inputs: &mut CallInputs,
    ) -> Option<(InterpreterResult, Range<usize>)> {
        self.immutable.call(data, inputs);
        self.mutable.call(data, inputs)
    }

    fn call_end(
        &mut self,
        context: &mut EvmContext<'_, DatabaseError>,
        result: InterpreterResult,
    ) -> InterpreterResult {
        self.immutable.call_end(context, result.clone());
        self.mutable.call_end(context, result)
    }

    fn create(
        &mut self,
        context: &mut EvmContext<'_, DatabaseError>,
        inputs: &mut CreateInputs,
    ) -> Option<(InterpreterResult, Option<Address>)> {
        self.immutable.create(context, inputs);
        self.mutable.create(context, inputs)
    }

    fn create_end(
        &mut self,
        context: &mut EvmContext<'_, DatabaseError>,
        result: InterpreterResult,
        address: Option<Address>,
    ) -> (InterpreterResult, Option<Address>) {
        self.immutable
            .create_end(context, result.clone(), address.clone());
        self.mutable.create_end(context, result, address)
    }

    fn selfdestruct(&mut self, contract: Address, target: Address, value: U256) {
        self.immutable.selfdestruct(contract, target, value);
        self.mutable.selfdestruct(contract, target, value);
    }
}

/// Container for storing inspector and tracer.
pub enum InspectorContainer<'inspector, BlockchainErrorT, StateErrorT>
where
    BlockchainErrorT: Debug,
    StateErrorT: Debug,
{
    /// No inspector or tracer.
    None,
    /// Only a tracer.
    Collector(TraceCollector),
    /// Both a tracer and an inspector.
    Dual(
        DualInspector<
            TraceCollector,
            &'inspector mut dyn SyncInspector<BlockchainErrorT, StateErrorT>,
            DatabaseComponentError<StateErrorT, BlockchainErrorT>,
        >,
    ),
    /// Only an inspector.
    Inspector(&'inspector mut dyn SyncInspector<BlockchainErrorT, StateErrorT>),
}

impl<'inspector, BlockchainErrorT, StateErrorT>
    InspectorContainer<'inspector, BlockchainErrorT, StateErrorT>
where
    BlockchainErrorT: Debug + Send,
    StateErrorT: Debug + Send,
{
    /// Constructs a new instance.
    pub fn new(
        with_trace: bool,
        tracer: Option<&'inspector mut dyn SyncInspector<BlockchainErrorT, StateErrorT>>,
    ) -> Self {
        if with_trace {
            if let Some(tracer) = tracer {
                InspectorContainer::Dual(DualInspector::new(TraceCollector::default(), tracer))
            } else {
                InspectorContainer::Collector(TraceCollector::default())
            }
        } else if let Some(tracer) = tracer {
            InspectorContainer::Inspector(tracer)
        } else {
            InspectorContainer::None
        }
    }

    /// Returns the inspector, if it exists.
    pub fn as_dyn_inspector(
        &mut self,
    ) -> Option<&mut dyn SyncInspector<BlockchainErrorT, StateErrorT>> {
        match self {
            InspectorContainer::None => None,
            InspectorContainer::Collector(c) => Some(c),
            InspectorContainer::Dual(d) => Some(d),
            InspectorContainer::Inspector(t) => Some(t),
        }
    }

    /// Returns the tracer, if it exists.
    pub fn into_tracer(self) -> Option<TraceCollector> {
        match self {
            InspectorContainer::None | InspectorContainer::Inspector(_) => None,
            InspectorContainer::Collector(c) => Some(c),
            InspectorContainer::Dual(d) => Some(d.into_parts().0),
        }
    }

    /// Clears and returns the trace, if it exists.
    pub fn clear_trace(&mut self) -> Option<Trace> {
        match self {
            InspectorContainer::None | InspectorContainer::Inspector(_) => None,
            InspectorContainer::Collector(collector) => {
                let tracer = std::mem::take(collector);
                Some(tracer.into_trace())
            }
            InspectorContainer::Dual(dual) => {
                let tracer = std::mem::take(&mut dual.immutable);
                Some(tracer.into_trace())
            }
        }
    }
}
