#![feature(assert_matches)]

mod common;
use common::*;

use std::{assert_matches::assert_matches, collections::HashMap};

use starknet_core::types::{FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError,
};

// invalid transaction_hash
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn fail_invalid_transaction_hash(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_transaction_receipt(FieldElement::ZERO)
        .await
        .err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(
            StarknetError::InvalidTransactionHash
        ))
    );
}

async fn work_with_hash(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
    transaction_hash: &str,
) {
    let transaction_hash =
        FieldElement::from_hex_be(transaction_hash).expect("Error parsing transaction hash");

    let response_deoxys = deoxys
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_transaction_receipt(transaction_hash)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

/// reverted transaction on block 200000
#[require(block_min = 200_000, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_reverted_transaction_block_200_000(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0x410e4d74a2322b78d2e342ac376ea555c89b1a0fe73bb36067eb149da123dd1",
    )
    .await;
}

/// first transaction on block 0
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_first_transaction_block_0(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0xe0a2e45a80bb827967e096bcf58874f6c01c191e0a0530624cba66a508ae75",
    )
    .await;
}

/// deploy transaction on block 0
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_deploy_transaction_block_0(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0x12c96ae3c050771689eb261c9bf78fac2580708c7f1f3d69a9647d8be59f1e1",
    )
    .await;
}

///invoke transaction on block 0
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_invoke_transaction_block_0(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0xce54bbc5647e1c1ea4276c01a708523f740db0ff5474c77734f73beec2624",
    )
    .await;
}

///deploy transaction on block 1
#[require(block_min = 1, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_deploy_transaction_block_1(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_hash(
        deoxys,
        pathfinder,
        "0x2f07a65f9f7a6445b2a0b1fb90ef12f5fd3b94128d06a67712efd3b2f163533",
    )
    .await;
}
