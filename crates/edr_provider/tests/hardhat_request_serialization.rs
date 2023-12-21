mod common;

use edr_eth::{Address, Bytes, B256, U256};
use edr_provider::{
    hardhat_rpc_types::{CompilerInput, CompilerOutput, ForkConfig, ResetProviderConfig},
    MethodInvocation,
};

use crate::common::help_test_method_invocation_serde;

#[test]
fn serde_hardhat_compiler() {
    // these were taken from a run of TypeScript function compileLiteral
    let compiler_input_json = include_str!("fixtures/compiler_input.json");
    let compiler_output_json = include_str!("fixtures/compiler_output.json");

    let call = MethodInvocation::AddCompilationResult(
        String::from("0.8.0"),
        serde_json::from_str::<CompilerInput>(compiler_input_json).unwrap(),
        serde_json::from_str::<CompilerOutput>(compiler_output_json).unwrap(),
    );

    help_test_method_invocation_serde(call.clone());

    match call {
        MethodInvocation::AddCompilationResult(_, ref input, ref output) => {
            assert_eq!(
                serde_json::to_value(input).unwrap(),
                serde_json::to_value(
                    serde_json::from_str::<CompilerInput>(compiler_input_json).unwrap()
                )
                .unwrap(),
            );
            assert_eq!(
                serde_json::to_value(output).unwrap(),
                serde_json::to_value(
                    serde_json::from_str::<CompilerOutput>(compiler_output_json).unwrap()
                )
                .unwrap(),
            );
        }
        _ => panic!("method invocation should have been AddCompilationResult"),
    }
}

#[test]
fn serde_hardhat_drop_transaction() {
<<<<<<< HEAD:crates/edr_rpc_hardhat/tests/hardhat.rs
    help_test_method_invocation_serde(edr_rpc_hardhat::Request::DropTransaction(B256::from(
        U256::from(1),
    )));
=======
    help_test_method_invocation_serde(MethodInvocation::DropTransaction(B256::from_low_u64_ne(1)));
>>>>>>> origin/edr/main:crates/edr_provider/tests/hardhat_request_serialization.rs
}

#[test]
fn serde_hardhat_get_automine() {
    help_test_method_invocation_serde(MethodInvocation::GetAutomine(()));
}

#[test]
fn serde_hardhat_get_stack_trace_failures_count() {
    help_test_method_invocation_serde(MethodInvocation::GetStackTraceFailuresCount(()));
}

#[test]
fn serde_hardhat_impersonate_account() {
<<<<<<< HEAD:crates/edr_rpc_hardhat/tests/hardhat.rs
    help_test_method_invocation_serde(edr_rpc_hardhat::Request::ImpersonateAccount(
        Address::random(),
=======
    help_test_method_invocation_serde(MethodInvocation::ImpersonateAccount(
        Address::from_low_u64_ne(1),
>>>>>>> origin/edr/main:crates/edr_provider/tests/hardhat_request_serialization.rs
    ));
}

#[test]
fn serde_hardhat_interval_mine() {
    help_test_method_invocation_serde(MethodInvocation::IntervalMine(()));
}

#[test]
fn serde_hardhat_metadata() {
    help_test_method_invocation_serde(MethodInvocation::Metadata(()));
}

#[test]
fn serde_hardhat_mine() {
    help_test_method_invocation_serde(MethodInvocation::Mine(Some(1), Some(1)));
    help_test_method_invocation_serde(MethodInvocation::Mine(Some(1), None));
    help_test_method_invocation_serde(MethodInvocation::Mine(None, Some(1)));
    help_test_method_invocation_serde(MethodInvocation::Mine(None, None));

    let json = r#"{"jsonrpc":"2.0","method":"hardhat_mine","params":[],"id":2}"#;
    let deserialized: MethodInvocation = serde_json::from_str(json)
        .unwrap_or_else(|_| panic!("should have successfully deserialized json {json}"));
    assert_eq!(MethodInvocation::Mine(None, None), deserialized);
}

#[test]
fn serde_hardhat_reset() {
    help_test_method_invocation_serde(MethodInvocation::Reset(Some(ResetProviderConfig {
        forking: Some(ForkConfig {
            json_rpc_url: String::from("http://whatever.com/whatever"),
            block_number: Some(123456),
            http_headers: None,
        }),
    })));
}

#[test]
fn serde_hardhat_set_balance() {
<<<<<<< HEAD:crates/edr_rpc_hardhat/tests/hardhat.rs
    help_test_method_invocation_serde(edr_rpc_hardhat::Request::SetBalance(
        Address::random(),
=======
    help_test_method_invocation_serde(MethodInvocation::SetBalance(
        Address::from_low_u64_ne(1),
>>>>>>> origin/edr/main:crates/edr_provider/tests/hardhat_request_serialization.rs
        U256::ZERO,
    ));
}

#[test]
fn serde_hardhat_set_code() {
<<<<<<< HEAD:crates/edr_rpc_hardhat/tests/hardhat.rs
    help_test_method_invocation_serde(edr_rpc_hardhat::Request::SetCode(
        Address::random(),
        Bytes::from(&b"whatever"[..]),
=======
    help_test_method_invocation_serde(MethodInvocation::SetCode(
        Address::from_low_u64_ne(1),
        Bytes::from(&b"whatever"[..]).into(),
>>>>>>> origin/edr/main:crates/edr_provider/tests/hardhat_request_serialization.rs
    ));
}

#[test]
fn serde_hardhat_set_coinbase() {
<<<<<<< HEAD:crates/edr_rpc_hardhat/tests/hardhat.rs
    help_test_method_invocation_serde(edr_rpc_hardhat::Request::SetCoinbase(Address::random()));
=======
    help_test_method_invocation_serde(MethodInvocation::SetCoinbase(Address::from_low_u64_ne(1)));
>>>>>>> origin/edr/main:crates/edr_provider/tests/hardhat_request_serialization.rs
}

#[test]
fn serde_hardhat_set_logging_enabled() {
    help_test_method_invocation_serde(MethodInvocation::SetLoggingEnabled(true));
}

#[test]
fn serde_hardhat_set_min_gas_price() {
    help_test_method_invocation_serde(MethodInvocation::SetMinGasPrice(U256::from(1)));
}

#[test]
fn serde_hardhat_set_next_block_base_fee_per_gas() {
    help_test_method_invocation_serde(MethodInvocation::SetNextBlockBaseFeePerGas(U256::from(1)));
}

#[test]
fn serde_hardhat_set_nonce() {
<<<<<<< HEAD:crates/edr_rpc_hardhat/tests/hardhat.rs
    help_test_method_invocation_serde(edr_rpc_hardhat::Request::SetNonce(Address::random(), 1u64));
=======
    help_test_method_invocation_serde(MethodInvocation::SetNonce(
        Address::from_low_u64_ne(1),
        1u64,
    ));
>>>>>>> origin/edr/main:crates/edr_provider/tests/hardhat_request_serialization.rs
}

#[test]
fn serde_hardhat_set_prev_randao() {
    help_test_method_invocation_serde(MethodInvocation::SetPrevRandao(B256::random()));
}

#[test]
fn serde_hardhat_set_storage_at() {
<<<<<<< HEAD:crates/edr_rpc_hardhat/tests/hardhat.rs
    help_test_method_invocation_serde(edr_rpc_hardhat::Request::SetStorageAt(
        Address::random(),
=======
    help_test_method_invocation_serde(MethodInvocation::SetStorageAt(
        Address::from_low_u64_ne(1),
>>>>>>> origin/edr/main:crates/edr_provider/tests/hardhat_request_serialization.rs
        U256::ZERO,
        U256::ZERO,
    ));
}

#[test]
fn serde_hardhat_stop_impersonating_account() {
<<<<<<< HEAD:crates/edr_rpc_hardhat/tests/hardhat.rs
    help_test_method_invocation_serde(edr_rpc_hardhat::Request::StopImpersonatingAccount(
        Address::random(),
=======
    help_test_method_invocation_serde(MethodInvocation::StopImpersonatingAccount(
        Address::from_low_u64_ne(1),
>>>>>>> origin/edr/main:crates/edr_provider/tests/hardhat_request_serialization.rs
    ));
}
