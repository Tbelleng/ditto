#![feature(assert_matches)]

use rpc_test::test_config::TestConfig;
use starknet_core::types::{
    BlockId, BlockTag, BroadcastedInvokeTransaction, BroadcastedTransaction, FieldElement,
    StarknetError,
};
use starknet_core::utils::get_selector_from_name;
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    MaybeUnknownErrorCode, Provider, ProviderError, StarknetErrorWithMessage,
};
use std::assert_matches::assert_matches;
use url::Url;

const TEST_CONTRACT_CLASS_HASH: &str = "";

#[tokio::test]
async fn fail_non_existing_block() {
    let config = TestConfig::new("./secret.json").unwrap();
    let deoxys = JsonRpcClient::new(HttpTransport::new(Url::parse(&config.deoxys).unwrap()));

    let test_contract_class_hash =
        FieldElement::from_hex_be(TEST_CONTRACT_CLASS_HASH).expect("Invalid Contract Address");

    assert_matches!(
        deoxys
        .get_class(
            BlockId::Number(100),
            test_contract_class_hash,
        )
        .await,
        Err(ProviderError::StarknetError(StarknetErrorWithMessage { code: MaybeUnknownErrorCode::Known(code), .. })) if code == StarknetError::BlockNotFound
    );
}

#[tokio::test]
async fn fail_non_existing_class_hash() {
    let config = TestConfig::new("./secret.json").unwrap();
    let deoxys = JsonRpcClient::new(HttpTransport::new(Url::parse(&config.deoxys).unwrap()));

    let unknown_contract_class_hash =
        FieldElement::from_hex_be("0x4269DEADBEEF").expect("Invalid Contract classh hash");

    assert_matches!(
        deoxys
        .get_class(
            BlockId::Number(0),
            unknown_contract_class_hash,
        )
        .await,
        Err(ProviderError::StarknetError(StarknetErrorWithMessage { code: MaybeUnknownErrorCode::Known(code), .. })) if code == StarknetError::ClassHashNotFound
    );
}

#[tokio::test]
async fn work_ok_retrieving_class_for_contract_version_0() {
    let config = TestConfig::new("./secret.json").unwrap();
    let deoxys = JsonRpcClient::new(HttpTransport::new(Url::parse(&config.deoxys).unwrap()));

    let test_contract_class_hash =
        FieldElement::from_hex_be(TEST_CONTRACT_CLASS_HASH).expect("Invalid Contract Class Hash");

    let test_contract_class_bytes = include_bytes!("../cairo-contracts/build/test.json");
    let test_contract_class: LegacyContractClass = serde_json::from_slice(test_contract_class_bytes).unwrap();

    assert_matches!(
        rpc
        .get_class(
            BlockId::Number(0),
            test_contract_class_hash,
        ).await?,
        ContractClass::Legacy(c) => {
            let mut gz = GzDecoder::new(&c.program[..]);
            let mut decompressed_bytes = Vec::new();
            gz.read_to_end(&mut decompressed_bytes).unwrap();
            let program: LegacyProgram = serde_json::from_slice(decompressed_bytes.as_slice())?;
            assert_eq!(
                program.data.len(),
                test_contract_class.program.data.len(),
            );
        }
    );

}