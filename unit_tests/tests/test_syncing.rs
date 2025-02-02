mod common;
use common::*;

use starknet_core::types::SyncStatusType;
use starknet_providers::{jsonrpc::HttpTransport, JsonRpcClient, Provider};
use std::collections::HashMap;

///
/// Unit test for `starknet_syncing`
///
/// purpose: returns starknet sync status
/// success case: sync status matches between providers (NOT DETERMINISTIC)
///
#[require(block_min = "latest", spec_version = "0.5.1")]
#[rstest]
#[tokio::test]
async fn syncing(clients: HashMap<String, JsonRpcClient<HttpTransport>>) {
    let deoxys = &clients[DEOXYS];
    let pathfinder = &clients[PATHFINDER];

    let response_deoxys = deoxys
        .syncing()
        .await
        .expect("Error while getting sync status from deoxys node");

    let response_pathfinder = pathfinder
        .syncing()
        .await
        .expect("Error while getting sync status from pathfinder node");

    assert!(compare_sync_status(response_deoxys, response_pathfinder));
}

/// compare 2 SyncStatus, only fields corresponding to current and highest block are compared
/// because the other fields are not deterministic and depend on restart of the node
fn compare_sync_status(a: SyncStatusType, b: SyncStatusType) -> bool {
    match (a, b) {
        (SyncStatusType::Syncing(a), SyncStatusType::Syncing(b)) => {
            a.current_block_num == b.current_block_num
                && a.current_block_hash == b.current_block_hash
                && a.highest_block_num == b.highest_block_num
                && a.highest_block_hash == b.highest_block_hash
        }
        (SyncStatusType::NotSyncing, SyncStatusType::NotSyncing) => true,
        _ => false,
    }
}
