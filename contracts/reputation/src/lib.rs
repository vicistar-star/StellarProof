#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env};

/// Per-creator reputation record stored on-chain
#[contracttype]
#[derive(Clone)]
pub struct ReputationRecord {
    pub score: i64,               // can go negative for bad actors
    pub total_submissions: u32,
    pub verified_authentic: u32,
    pub flagged_synthetic: u32,
    pub disputed: u32,
    pub last_updated: u64,
}

#[contracttype]
pub enum DataKey {
    Reputation(Address),
}

/// Score deltas applied per event
const AUTHENTIC_DELTA: i64 = 10;
const SYNTHETIC_DELTA: i64 = -15;
const DISPUTE_DELTA: i64 = -5;
const DISPUTE_RESOLVED_UPHELD_DELTA: i64 = 5; // restored if challenge fails

#[contract]
pub struct ReputationContract;

#[contractimpl]
impl ReputationContract {
    /// Record a verified-authentic submission — increments score.
    pub fn record_authentic(env: Env, oracle: Address, creator: Address) {
        oracle.require_auth();
        let mut rec = Self::get_or_default(&env, &creator);
        rec.score += AUTHENTIC_DELTA;
        rec.total_submissions += 1;
        rec.verified_authentic += 1;
        rec.last_updated = env.ledger().timestamp();
        env.storage()
            .persistent()
            .set(&DataKey::Reputation(creator), &rec);
    }

    /// Record a flagged-synthetic submission — decrements score.
    pub fn record_synthetic(env: Env, oracle: Address, creator: Address) {
        oracle.require_auth();
        let mut rec = Self::get_or_default(&env, &creator);
        rec.score += SYNTHETIC_DELTA;
        rec.total_submissions += 1;
        rec.flagged_synthetic += 1;
        rec.last_updated = env.ledger().timestamp();
        env.storage()
            .persistent()
            .set(&DataKey::Reputation(creator), &rec);
    }

    /// Record an open dispute against a creator's certificate.
    pub fn record_dispute(env: Env, oracle: Address, creator: Address) {
        oracle.require_auth();
        let mut rec = Self::get_or_default(&env, &creator);
        rec.score += DISPUTE_DELTA;
        rec.disputed += 1;
        rec.last_updated = env.ledger().timestamp();
        env.storage()
            .persistent()
            .set(&DataKey::Reputation(creator), &rec);
    }

    /// Restore partial score when a challenge is resolved as upheld (false challenge).
    pub fn resolve_dispute_upheld(env: Env, oracle: Address, creator: Address) {
        oracle.require_auth();
        let mut rec = Self::get_or_default(&env, &creator);
        rec.score += DISPUTE_RESOLVED_UPHELD_DELTA;
        rec.last_updated = env.ledger().timestamp();
        env.storage()
            .persistent()
            .set(&DataKey::Reputation(creator), &rec);
    }

    /// Get reputation record for a creator.
    pub fn get_reputation(env: Env, creator: Address) -> ReputationRecord {
        Self::get_or_default(&env, &creator)
    }

    fn get_or_default(env: &Env, creator: &Address) -> ReputationRecord {
        env.storage()
            .persistent()
            .get(&DataKey::Reputation(creator.clone()))
            .unwrap_or(ReputationRecord {
                score: 0,
                total_submissions: 0,
                verified_authentic: 0,
                flagged_synthetic: 0,
                disputed: 0,
                last_updated: env.ledger().timestamp(),
            })
    }
}
