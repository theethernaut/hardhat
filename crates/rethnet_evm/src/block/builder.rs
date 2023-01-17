use std::{fmt::Debug, sync::Arc};

use rethnet_eth::{
    block::{Header, PartialHeader},
    Address, U256,
};
use revm::{BlockEnv, CfgEnv, ExecutionResult, SpecId, TxEnv};
use tokio::runtime::Runtime;

use crate::{
    blockchain::AsyncBlockchain, db::AsyncDatabase, evm::build_evm, inspector::RethnetInspector,
    trace::Trace, HeaderData,
};

#[derive(Debug, thiserror::Error)]
pub enum BlockTransactionError {
    #[error("Transaction has a higher gas limit than the remaining gas in the block")]
    ExceedsBlockGasLimit,
}

/// A builder for constructing Ethereum blocks.
pub struct BlockBuilder<BE, DE>
where
    BE: Debug + Send + 'static,
    DE: Debug + Send + 'static,
{
    blockchain: Arc<AsyncBlockchain<BE>>,
    state: Arc<AsyncDatabase<DE>>,
    header: PartialHeader,
    transactions: Vec<TxEnv>,
    cfg: CfgEnv,
}

impl<BE, DE> BlockBuilder<BE, DE>
where
    BE: Debug + Send + 'static,
    DE: Debug + Send + 'static,
{
    /// Creates an intance of [`BlockBuilder`], creating a checkpoint in the process.
    pub async fn new(
        blockchain: Arc<AsyncBlockchain<BE>>,
        db: Arc<AsyncDatabase<DE>>,
        cfg: CfgEnv,
        parent: Header,
        header: HeaderData,
    ) -> Self {
        // TODO: Proper implementation of a block builder
        // db.checkpoint().await?;

        // TODO: Allow user to pass in values
        let header = PartialHeader {
            parent_hash: header.parent_hash.unwrap_or(parent.parent_hash),
            number: header.number.unwrap_or(parent.number + U256::from(1)),
            gas_limit: header.gas_limit.unwrap_or(parent.gas_limit),
            ..PartialHeader::default()
        };

        Self {
            blockchain,
            state: db,
            header,
            transactions: Vec::new(),
            cfg,
        }
    }

    /// Retrieves the runtime of the [`BlockBuilder`].
    pub fn runtime(&self) -> &Runtime {
        self.state.runtime()
    }

    /// Retrieves the amount of gas used in the block, so far.
    pub fn gas_used(&self) -> U256 {
        self.header.gas_used
    }

    /// Retrieves the amount of gas left in the block.
    pub fn gas_remaining(&self) -> U256 {
        self.header.gas_limit - self.gas_used()
    }

    // fn miner_reward(num_ommers: u64) -> U256 {
    //     // TODO: This is the LONDON block reward. Did it change?
    //     const BLOCK_REWARD: u64 = 2 * 10u64.pow(18);
    //     const NIBLING_REWARD: u64 = BLOCK_REWARD / 32;

    //     U256::from(BLOCK_REWARD + num_ommers * NIBLING_REWARD)
    // }

    /// Adds a pending transaction to
    pub async fn add_transaction(
        &mut self,
        transaction: TxEnv,
    ) -> Result<(ExecutionResult, Trace), BlockTransactionError> {
        //  transaction's gas limit cannot be greater than the remaining gas in the block
        if U256::from(transaction.gas_limit) > self.gas_remaining() {
            return Err(BlockTransactionError::ExceedsBlockGasLimit);
        }

        self.transactions.push(transaction.clone());
        let block = BlockEnv {
            number: self.header.number,
            coinbase: self.header.beneficiary,
            timestamp: U256::from(self.header.timestamp),
            difficulty: self.header.difficulty,
            basefee: self.header.base_fee.unwrap_or(U256::ZERO),
            gas_limit: self.header.gas_limit,
            prevrandao: if self.cfg.spec_id > SpecId::MERGE {
                Some(self.header.mix_hash)
            } else {
                None
            },
        };

        let blockchain = self.blockchain.clone();
        let db = self.state.clone();
        let cfg = self.cfg.clone();

        let (result, changes, trace) = self
            .state
            .runtime()
            .spawn(async move {
                let mut evm = build_evm(&blockchain, &db, cfg, transaction, block);

                let mut inspector = RethnetInspector::default();
                let (result, state) = evm.inspect(&mut inspector);
                (result, state, inspector.into_trace())
            })
            .await
            .unwrap();

        self.state.apply(changes).await;

        self.header.gas_used += U256::from(result.gas_used);

        // TODO: store receipt
        Ok((result, trace))
    }

    /// Finalizes the block, returning the state root.
    /// TODO: Build a full block
    pub async fn finalize(self, rewards: Vec<(Address, U256)>) -> Result<(), DE> {
        for (address, reward) in rewards {
            self.state
                .modify_account(
                    address,
                    Box::new(move |balance, _nonce, _code| *balance += reward),
                )
                .await?;
        }

        Ok(())
    }

    /// Aborts building of the block, reverting all transactions in the process.
    pub async fn abort(self) -> Result<(), DE> {
        self.state.revert().await
    }
}
