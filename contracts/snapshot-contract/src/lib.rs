#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Bytes, Env, Map};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snapshot {
    pub hash: Bytes,
    pub epoch: u64,
    pub timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    Snapshots,
}

#[contract]
pub struct SnapshotContract;

#[contractimpl]
impl SnapshotContract {
    /// Submit a snapshot hash for verification
    ///
    /// # Arguments
    /// * `hash` - The analytics hash to store
    /// * `epoch` - The epoch identifier for the snapshot
    ///
    /// # Returns
    /// The timestamp when the snapshot was submitted
    pub fn submit_snapshot(env: Env, hash: Bytes, epoch: u64) -> u64 {
        let timestamp = env.ledger().timestamp();

        // Create snapshot
        let snapshot = Snapshot {
            hash: hash.clone(),
            epoch,
            timestamp,
        };

        // Store snapshot in persistent storage
        let mut snapshots: Map<u64, Snapshot> = env
            .storage()
            .persistent()
            .get(&DataKey::Snapshots)
            .unwrap_or(Map::new(&env));

        snapshots.set(epoch, snapshot);
        env.storage()
            .persistent()
            .set(&DataKey::Snapshots, &snapshots);

        // Emit event
        env.events()
            .publish((symbol_short!("SNAP_SUB"),), (hash, epoch, timestamp));

        timestamp
    }

    /// Get a snapshot by epoch
    ///
    /// # Arguments
    /// * `epoch` - The epoch identifier
    ///
    /// # Returns
    /// The snapshot data if it exists
    pub fn get_snapshot(env: Env, epoch: u64) -> Option<Snapshot> {
        let snapshots: Map<u64, Snapshot> = env
            .storage()
            .persistent()
            .get(&DataKey::Snapshots)
            .unwrap_or(Map::new(&env));

        snapshots.get(epoch)
    }
}

mod test;
