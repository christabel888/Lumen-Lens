#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, BytesN, Env, Map};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotMetadata {
    pub id: u64,
    pub timestamp: u64,
    pub hash: BytesN<32>,
    // Extendable for future fields
}

#[contracttype]
pub enum DataKey {
    Latest,
    History,
}

#[contract]
pub struct AnalyticsContract;

#[contractimpl]
impl AnalyticsContract {
    /// Initialize contract storage for latest snapshot and history
    pub fn initialize(env: Env) {
        let storage = env.storage().instance();
        if storage
            .get::<_, SnapshotMetadata>(&DataKey::Latest)
            .is_none()
        {
            // No latest snapshot yet
            storage.set(&DataKey::Latest, &None::<SnapshotMetadata>);
        }
        if storage
            .get::<_, Map<u64, SnapshotMetadata>>(&DataKey::History)
            .is_none()
        {
            // Empty history map
            let empty = Map::<u64, SnapshotMetadata>::new(&env);
            storage.set(&DataKey::History, &empty);
        }
    }

    /// Set the latest snapshot and append to history
    pub fn set_snapshot(env: Env, metadata: SnapshotMetadata) {
        let storage = env.storage().instance();
        // Set latest
        storage.set(&DataKey::Latest, &Some(metadata.clone()));
        // Append to history
        let mut history = storage
            .get::<_, Map<u64, SnapshotMetadata>>(&DataKey::History)
            .unwrap_or_else(|| Map::new(&env));
        history.set(metadata.id, metadata);
        storage.set(&DataKey::History, &history);
    }

    /// Get the latest snapshot
    pub fn get_latest_snapshot(env: Env) -> Option<SnapshotMetadata> {
        env.storage()
            .instance()
            .get(&DataKey::Latest)
            .unwrap_or(None)
    }

    /// Get the full snapshot history as a Map
    pub fn get_snapshot_history(env: Env) -> Map<u64, SnapshotMetadata> {
        env.storage()
            .instance()
            .get(&DataKey::History)
            .unwrap_or_else(|| Map::new(&env))
    }
}
