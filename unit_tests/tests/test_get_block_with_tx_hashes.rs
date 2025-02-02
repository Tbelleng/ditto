#![feature(assert_matches)]

mod common;
use common::*;
use starknet_core::types::MaybePendingBlockWithTxHashes;

use std::sync::Arc;
use std::{assert_matches::assert_matches, collections::HashMap};

use starknet_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
use starknet_providers::{
    jsonrpc::{HttpTransport, JsonRpcClient},
    Provider, ProviderError,
};
use unit_tests::constants::DEOXYS;

///
/// Unit test for `starknet_get_block_with_tx_hashes`
///
/// purpose: call getBlockWithTxHashes on invalid block.
/// fail case: invalid block.
///
#[require(spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn fail_non_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];

    let response_deoxys = deoxys
        .get_block_with_tx_hashes(BlockId::Hash(FieldElement::ZERO))
        .await
        .err();

    assert_matches!(
        response_deoxys,
        Some(ProviderError::StarknetError(StarknetError::BlockNotFound))
    );
}

///
/// Unit test for `starknet_get_block_with_tx_hashes`
///
/// purpose: call getBlockWithTxHashes on latest validated block.
/// success case: retrieves valid block.
///
#[require(block_min = "latest", spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_existing_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await
        .expect("Error waiting for response from Deoxys node");
    let response_pathfinder = pathfinder
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest))
        .await
        .expect("Error waiting for response from Deoxys node");

    let block_deoxys = match response_deoxys {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        MaybePendingBlockWithTxHashes::PendingBlock(_) => {
            panic!("Expected block, got pending block")
        }
    };
    let block_pathfinder = match response_pathfinder {
        MaybePendingBlockWithTxHashes::Block(block) => block,
        MaybePendingBlockWithTxHashes::PendingBlock(_) => {
            panic!("Expected block, got pending block")
        }
    };

    assert_eq!(block_deoxys, block_pathfinder);
}

///
/// Unit test for `starknet_get_block_with_tx_hashes`
///
/// purpose: call getBlockWithTxHashes on pending block.
/// success case: retrieves valid pending block.
///
/// Note that this can fail at the last moments of a block being validated!!!
///
#[require(block_min = "latest", spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
#[ignore = "Pending fails some times when called on the cusp of being accepted, need virtual sequencer"]
async fn work_pending_block(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Pending))
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_with_tx_hashes(BlockId::Tag(BlockTag::Pending))
        .await
        .expect("Error waiting for response from Deoxys node");

    let block_deoxys = match response_deoxys {
        MaybePendingBlockWithTxHashes::Block(_) => panic!("Expected pending block, got block"),
        MaybePendingBlockWithTxHashes::PendingBlock(block) => block,
    };
    let block_pathfinder = match response_pathfinder {
        MaybePendingBlockWithTxHashes::Block(_) => panic!("Expected pending block, got block"),
        MaybePendingBlockWithTxHashes::PendingBlock(block) => block,
    };

    assert_eq!(block_deoxys, block_pathfinder);
}

async fn work_with_block(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
    block_number: u64,
) {
    let block_number = BlockId::Number(block_number);

    let response_deoxys = deoxys
        .get_block_with_tx_hashes(block_number)
        .await
        .expect("Error waiting for response from Deoxys node");

    let response_pathfinder = pathfinder
        .get_block_with_tx_hashes(block_number)
        .await
        .expect("Error waiting for response from Pathfinder node");

    assert_eq!(response_deoxys, response_pathfinder);
}

/// block 1
#[require(block_min = 1, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_block_1(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_block(deoxys, pathfinder, 1).await;
}

/// block 3800 is the first block with starknet_version in the header
#[require(block_min = 3800, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_block_3800(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_block(deoxys, pathfinder, 1).await;
}

/// block 50066 is one of the biggest blocks in the mainnet
#[require(block_min = 5066, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_block_5066(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_block(deoxys, pathfinder, 1).await;
}

/// block 1466-2242 mismatch block_hash
#[require(block_min = 1500, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn work_with_block_1500(
    deoxys: JsonRpcClient<HttpTransport>,
    pathfinder: JsonRpcClient<HttpTransport>,
) {
    work_with_block(deoxys, pathfinder, 1500).await;
}

#[require(block_min = 100_000, spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
#[ignore = "ignore this test"]
async fn work_loop(deoxys: JsonRpcClient<HttpTransport>, pathfinder: JsonRpcClient<HttpTransport>) {
    let arc_deoxys = Arc::new(deoxys);
    let arc_pathfinder = Arc::new(pathfinder);
    let parallels_queries = 10;
    let mut diff = false;

    for block_group in (0..=100_000).step_by(parallels_queries) {
        let mut set = tokio::task::JoinSet::new();
        for offset in 0..parallels_queries {
            let block_id = (block_group + offset) as u64;
            let block = BlockId::Number(block_id);
            let clone_deoxys = Arc::clone(&arc_deoxys);
            let clone_pathfinder = Arc::clone(&arc_pathfinder);
            set.spawn(async move {
                let response_deoxys = clone_deoxys
                    .get_block_with_tx_hashes(block)
                    .await
                    .expect("Error waiting for response from Deoxys node");

                let response_pathfinder = clone_pathfinder.get_block_with_tx_hashes(block).await;

                match response_pathfinder {
                    Ok(response_pathfinder) => {
                        if response_deoxys != response_pathfinder {
                            Err(format!("block {}", block_id))
                        } else {
                            Ok(())
                        }
                    }
                    Err(e) => Err(format!("Error pathfinder: {}", e)),
                }
            });
        }
        while let Some(result) = set.join_next().await {
            match result {
                Ok(result) => match result {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                        diff = true;
                    }
                },
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
    }
    assert_eq!(diff, false);
}
