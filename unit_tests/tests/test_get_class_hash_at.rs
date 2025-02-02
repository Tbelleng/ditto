#![feature(assert_matches)]

mod common;
use common::*;

use std::{assert_matches::assert_matches, collections::HashMap};

use starknet_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider, ProviderError};

///
/// Unit test for `starknet_getClassHashAt`
///
/// purpose: call getClassHashAt on invalid block.
/// fail case: invalid block hash.
///
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_class_hash_at(
            BlockId::Hash(FieldElement::ZERO),
            FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap(),
        )
        .await
        .err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetError::BlockNotFound))
    )
}

///
/// Unit test for `starknet_getClassHashAt`
///
/// purpose: call getClassHashAt on non-existent contract.
/// fail case: invalid contract hash.
///
#[require(block_min = "latest", spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn fail_non_existing_contract(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_class_hash_at(
            BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(INVALID_CONTRACT_ADDR).unwrap(),
        )
        .await
        .err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(
            StarknetError::ContractNotFound
        ))
    )
}

///
/// Unit test for `starknet_getClassHashAt`
///
/// purpose: call getClassHashAt on latest block.
/// success case: retrieve valid class hash.
///
#[require(block_min = "latest", spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_block_latest(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let class_hash_deoxys = deoxys
        .get_class_hash_at(
            BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap(),
        )
        .await
        .expect("Error waiting for response from Deoxys node");
    let class_hash_pathfinder = pathfinder
        .get_class_hash_at(
            BlockId::Tag(BlockTag::Latest),
            FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap(),
        )
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(class_hash_deoxys, class_hash_pathfinder);
}

///
/// Unit test for `starknet_getClassHashAt`
///
/// purpose: call getClassHashAt on pending block.
/// success case: retrieve valid class hash.
///
#[require(block_min = "latest", spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
#[ignore = "Pending fails some times when called on the cusp of being accepted, need virtual sequencer"]
async fn work_block_pending(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let class_hash_deoxys = deoxys
        .get_class_hash_at(
            BlockId::Tag(BlockTag::Pending),
            FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap(),
        )
        .await
        .expect("Error waiting for response from Deoxys node");
    let class_hash_pathfinder = pathfinder
        .get_class_hash_at(
            BlockId::Tag(BlockTag::Pending),
            FieldElement::from_hex_be(STARKGATE_ETH_CONTRACT_ADDR).unwrap(),
        )
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(class_hash_deoxys, class_hash_pathfinder);
}
