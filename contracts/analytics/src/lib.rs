#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, Map, Vec};

const DEFAULT_SNAPSHOT_TTL: u64 = 7_776_000; // 90 days in seconds
const LEDGER_SECONDS: u64 = 5; // ~5 seconds per ledger

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotMetadata {
    pub epoch: u64,
    pub timestamp: u64,
    pub hash: BytesN<32>,
    pub submitter: Address,
    pub ledger_sequence: u32,
    pub expires_at: Option<u64>,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Snapshots,
    LatestEpoch,
    Snapshot(u64),
    Paused,
    Governance,
}

// ── Private helpers ──────────────────────────────────────────────────────────

/// Read the admin address; panics if the contract is not initialized.
fn require_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .unwrap_or_else(|| panic!("Contract not initialized: admin not set"))
}

/// Validate epoch ordering and return the current latest epoch.
fn validate_epoch(env: &Env, epoch: u64) -> u64 {
    if epoch == 0 {
        panic!("Invalid epoch: must be greater than 0");
    }
    let latest: u64 = env
        .storage()
        .instance()
        .get(&DataKey::LatestEpoch)
        .unwrap_or(0);
    if epoch == latest {
        panic!("Snapshot for epoch {} already exists", epoch);
    }
    if epoch < latest {
        panic!(
            "Epoch monotonicity violated: epoch {} must be strictly greater than latest {}",
            epoch, latest
        );
    }
    latest
}

/// Write one snapshot to per-epoch persistent storage and update the shared map + latest epoch.
/// `snapshots` is the already-loaded shared map (passed in to avoid a redundant read).
fn write_snapshot(
    env: &Env,
    epoch: u64,
    metadata: &SnapshotMetadata,
    snapshots: &mut Map<u64, SnapshotMetadata>,
) {
    // Per-epoch key — cheap individual read/write, used by get_snapshot
    env.storage()
        .persistent()
        .set(&DataKey::Snapshot(epoch), metadata);
    // Shared map — kept for get_snapshot_history / get_all_epochs
    snapshots.set(epoch, metadata.clone());
    env.storage()
        .persistent()
        .set(&DataKey::Snapshots, snapshots);
    env.storage().instance().set(&DataKey::LatestEpoch, &epoch);
}

// ── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct AnalyticsContract;

#[contractimpl]
impl AnalyticsContract {
    pub fn initialize(env: Env, admin: Address) {
        let storage = env.storage().instance();
        if storage.has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        storage.set(&DataKey::Admin, &admin);
        storage.set(&DataKey::LatestEpoch, &0u64);
        storage.set(&DataKey::Paused, &false);
        env.storage().persistent().set(
            &DataKey::Snapshots,
            &Map::<u64, SnapshotMetadata>::new(&env),
        );
    }

    /// Submit a single snapshot. Panics on invalid input or unauthorized caller.
    pub fn submit_snapshot(env: Env, epoch: u64, hash: BytesN<32>, caller: Address) -> u64 {
        if env
            .storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false)
        {
            panic!("Contract is paused for emergency maintenance");
        }
        caller.require_auth();
        let admin = require_admin(&env);
        if caller != admin {
            panic!("Unauthorized: only the admin can submit snapshots");
        }
        validate_epoch(&env, epoch);

        let timestamp = env.ledger().timestamp();
        let metadata = SnapshotMetadata {
            epoch,
            timestamp,
            hash,
            submitter: caller,
            ledger_sequence: env.ledger().sequence(),
            expires_at: None,
        };
        let mut snapshots: Map<u64, SnapshotMetadata> = env
            .storage()
            .persistent()
            .get(&DataKey::Snapshots)
            .unwrap_or_else(|| Map::new(&env));
        write_snapshot(&env, epoch, &metadata, &mut snapshots);
        timestamp
    }

    /// Submit a batch of snapshots in a single call (single auth + admin check).
    /// Epochs must be provided in strictly increasing order.
    /// Returns a Vec of timestamps, one per submitted snapshot.
    pub fn batch_submit(
        env: Env,
        snapshots_input: Vec<(u64, BytesN<32>)>,
        caller: Address,
    ) -> Vec<u64> {
        if env
            .storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false)
        {
            panic!("Contract is paused for emergency maintenance");
        }
        caller.require_auth();
        let admin = require_admin(&env);
        if caller != admin {
            panic!("Unauthorized: only the admin can submit snapshots");
        }

        let mut snapshots: Map<u64, SnapshotMetadata> = env
            .storage()
            .persistent()
            .get(&DataKey::Snapshots)
            .unwrap_or_else(|| Map::new(&env));

        let mut results = Vec::new(&env);
        for (epoch, hash) in snapshots_input.iter() {
            validate_epoch(&env, epoch);
            let timestamp = env.ledger().timestamp();
            let metadata = SnapshotMetadata {
                epoch,
                timestamp,
                hash,
                submitter: caller.clone(),
                ledger_sequence: env.ledger().sequence(),
                expires_at: None,
            };
            write_snapshot(&env, epoch, &metadata, &mut snapshots);
            results.push_back(timestamp);
        }
        results
    }

    /// Submit a snapshot with an optional TTL (defaults to 90 days).
    pub fn submit_snapshot_with_ttl(
        env: Env,
        epoch: u64,
        hash: BytesN<32>,
        caller: Address,
        ttl_seconds: Option<u64>,
    ) -> u64 {
        caller.require_auth();
        let admin = require_admin(&env);
        if caller != admin {
            panic!("Unauthorized: only the admin can submit snapshots");
        }
        validate_epoch(&env, epoch);

        let timestamp = env.ledger().timestamp();
        let ttl = ttl_seconds.unwrap_or(DEFAULT_SNAPSHOT_TTL);
        let metadata = SnapshotMetadata {
            epoch,
            timestamp,
            hash,
            submitter: caller,
            ledger_sequence: env.ledger().sequence(),
            expires_at: Some(timestamp + ttl),
        };

        let mut snapshots: Map<u64, SnapshotMetadata> = env
            .storage()
            .persistent()
            .get(&DataKey::Snapshots)
            .unwrap_or_else(|| Map::new(&env));
        write_snapshot(&env, epoch, &metadata, &mut snapshots);

        let ledgers_to_live = (ttl / LEDGER_SECONDS) as u32;
        env.storage().persistent().extend_ttl(
            &DataKey::Snapshot(epoch),
            ledgers_to_live,
            ledgers_to_live,
        );
        timestamp
    }

    /// Get snapshot by epoch — reads the cheap per-epoch key, not the full map.
    pub fn get_snapshot(env: Env, epoch: u64) -> Option<SnapshotMetadata> {
        env.storage().persistent().get(&DataKey::Snapshot(epoch))
    }

    pub fn get_latest_snapshot(env: Env) -> Option<SnapshotMetadata> {
        let latest_epoch: u64 = env
            .storage()
            .instance()
            .get(&DataKey::LatestEpoch)
            .unwrap_or(0);
        if latest_epoch == 0 {
            return None;
        }
        env.storage()
            .persistent()
            .get(&DataKey::Snapshot(latest_epoch))
    }

    pub fn get_snapshot_history(env: Env) -> Map<u64, SnapshotMetadata> {
        env.storage()
            .persistent()
            .get(&DataKey::Snapshots)
            .unwrap_or_else(|| Map::new(&env))
    }

    pub fn get_latest_epoch(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::LatestEpoch)
            .unwrap_or(0)
    }

    pub fn get_all_epochs(env: Env) -> soroban_sdk::Vec<u64> {
        let snapshots = Self::get_snapshot_history(env.clone());
        let mut epochs = soroban_sdk::Vec::new(&env);
        for (epoch, _) in snapshots.iter() {
            epochs.push_back(epoch);
        }
        epochs
    }

    pub fn get_admin(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::Admin)
    }

    pub fn set_admin(env: Env, current_admin: Address, new_admin: Address) {
        current_admin.require_auth();
        let admin = require_admin(&env);
        if current_admin != admin {
            panic!("Unauthorized: only the current admin can set a new admin");
        }
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }

    pub fn cleanup_expired_snapshots(env: Env, admin: Address, max_to_clean: u32) -> u32 {
        admin.require_auth();
        let stored_admin = require_admin(&env);
        if admin != stored_admin {
            panic!("Unauthorized: only the admin can clean up snapshots");
        }

        let now = env.ledger().timestamp();
        let mut cleaned = 0u32;
        let latest_epoch: u64 = env
            .storage()
            .instance()
            .get(&DataKey::LatestEpoch)
            .unwrap_or(0);

        let mut snapshots: Map<u64, SnapshotMetadata> = env
            .storage()
            .persistent()
            .get(&DataKey::Snapshots)
            .unwrap_or_else(|| Map::new(&env));

        for epoch in 1..=latest_epoch {
            if cleaned >= max_to_clean {
                break;
            }
            if let Some(metadata) = snapshots.get(epoch) {
                if let Some(expires_at) = metadata.expires_at {
                    if now > expires_at {
                        snapshots.remove(epoch);
                        env.storage().persistent().remove(&DataKey::Snapshot(epoch));
                        cleaned += 1;
                    }
                }
            }
        }

        env.storage()
            .persistent()
            .set(&DataKey::Snapshots, &snapshots);
        cleaned
    }

    pub fn is_snapshot_expired(env: Env, epoch: u64) -> bool {
        match env
            .storage()
            .persistent()
            .get::<DataKey, SnapshotMetadata>(&DataKey::Snapshot(epoch))
        {
            Some(metadata) => match metadata.expires_at {
                Some(expires_at) => env.ledger().timestamp() > expires_at,
                None => false,
            },
            None => false,
        }
    }

    pub fn pause(env: Env, caller: Address) {
        caller.require_auth();
        let admin = require_admin(&env);
        if caller != admin {
            panic!("Unauthorized: only the admin can pause the contract");
        }
        env.storage().instance().set(&DataKey::Paused, &true);
    }

    pub fn unpause(env: Env, caller: Address) {
        caller.require_auth();
        let admin = require_admin(&env);
        if caller != admin {
            panic!("Unauthorized: only the admin can unpause the contract");
        }
        env.storage().instance().set(&DataKey::Paused, &false);
    }

    pub fn set_governance(env: Env, caller: Address, governance: Address) {
        caller.require_auth();
        let admin = require_admin(&env);
        if caller != admin {
            panic!("Unauthorized: only the admin can set governance");
        }
        env.storage()
            .instance()
            .set(&DataKey::Governance, &governance);
    }

    pub fn get_governance(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::Governance)
    }

    pub fn set_admin_by_governance(env: Env, caller: Address, new_admin: Address) {
        let governance: Address = env
            .storage()
            .instance()
            .get(&DataKey::Governance)
            .unwrap_or_else(|| panic!("Governance not set"));
        if caller != governance {
            panic!("Unauthorized: only the governance contract can set admin");
        }
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }

    pub fn set_paused_by_governance(env: Env, caller: Address, paused: bool) {
        let governance: Address = env
            .storage()
            .instance()
            .get(&DataKey::Governance)
            .unwrap_or_else(|| panic!("Governance not set"));
        if caller != governance {
            panic!("Unauthorized: only the governance contract can set paused");
        }
        env.storage().instance().set(&DataKey::Paused, &paused);
    }

    pub fn is_paused(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests;
