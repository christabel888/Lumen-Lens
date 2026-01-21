#![cfg(test)]

use super::*;
use soroban_sdk::{bytes, testutils::Events, Env};

#[test]
fn test_submit_snapshot() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SnapshotContract);
    let client = SnapshotContractClient::new(&env, &contract_id);

    let hash = bytes!(&env, 0x1234567890abcdef);
    let epoch = 42u64;

    // Submit snapshot
    let timestamp = client.submit_snapshot(&hash, &epoch);

    // Verify snapshot was stored
    let stored_snapshot = client.get_snapshot(&epoch).unwrap();
    assert_eq!(stored_snapshot.hash, hash);
    assert_eq!(stored_snapshot.epoch, epoch);
    assert_eq!(stored_snapshot.timestamp, timestamp);
}

#[test]
fn test_snapshot_submitted_event() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SnapshotContract);
    let client = SnapshotContractClient::new(&env, &contract_id);

    let hash = bytes!(&env, 0xabcdef1234567890);
    let epoch = 100u64;

    // Submit snapshot
    client.submit_snapshot(&hash, &epoch);

    // Check event was emitted
    let events = env.events().all();
    assert_eq!(events.len(), 1);

    let (event_contract_id, _, _) = events.get(0).unwrap();
    assert_eq!(event_contract_id, contract_id);
}

#[test]
fn test_get_nonexistent_snapshot() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SnapshotContract);
    let client = SnapshotContractClient::new(&env, &contract_id);

    let epoch = 999u64;
    let snapshot = client.get_snapshot(&epoch);
    assert!(snapshot.is_none());
}

#[test]
fn test_multiple_snapshots() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SnapshotContract);
    let client = SnapshotContractClient::new(&env, &contract_id);

    // Submit first snapshot
    let hash1 = bytes!(&env, 0x1111111111111111);
    let epoch1 = 1u64;
    let timestamp1 = client.submit_snapshot(&hash1, &epoch1);

    // Submit second snapshot
    let hash2 = bytes!(&env, 0x2222222222222222);
    let epoch2 = 2u64;
    let timestamp2 = client.submit_snapshot(&hash2, &epoch2);

    // Verify both snapshots
    let snapshot1 = client.get_snapshot(&epoch1).unwrap();
    assert_eq!(snapshot1.hash, hash1);
    assert_eq!(snapshot1.epoch, epoch1);
    assert_eq!(snapshot1.timestamp, timestamp1);

    let snapshot2 = client.get_snapshot(&epoch2).unwrap();
    assert_eq!(snapshot2.hash, hash2);
    assert_eq!(snapshot2.epoch, epoch2);
    assert_eq!(snapshot2.timestamp, timestamp2);

    // Timestamps may be the same in test environment, which is acceptable
}
