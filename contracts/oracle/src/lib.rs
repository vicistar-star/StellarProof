#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Bytes, Env, String,
};

/// Status of a detection job
/// 0=QUEUED 1=PROCESSING 2=COMPLETE 3=FAILED
#[contracttype]
#[derive(Clone)]
pub struct DetectionJob {
    pub job_id: String,
    pub content_hash: String,
    pub creator: Address,
    pub status: u32,
    pub created_at: u64,
    pub completed_at: u64,  // 0 if not yet complete
    pub cert_id: String,    // empty if not yet complete
    pub attestation: Bytes, // TEE attestation document
}

#[contracttype]
pub enum DataKey {
    Job(String),
    Admin,
}

#[contract]
pub struct OracleContract;

#[contractimpl]
impl OracleContract {
    /// Initialize with an admin address (the TEE oracle worker key).
    pub fn initialize(env: Env, admin: Address) {
        assert!(
            !env.storage().instance().has(&symbol_short!("admin")),
            "already initialized"
        );
        env.storage()
            .instance()
            .set(&symbol_short!("admin"), &admin);
    }

    /// Creator submits a detection job on-chain.
    pub fn submit_job(
        env: Env,
        creator: Address,
        job_id: String,
        content_hash: String,
    ) -> String {
        creator.require_auth();

        let job = DetectionJob {
            job_id: job_id.clone(),
            content_hash,
            creator,
            status: 0, // QUEUED
            created_at: env.ledger().timestamp(),
            completed_at: 0,
            cert_id: String::from_str(&env, ""),
            attestation: Bytes::new(&env),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Job(job_id.clone()), &job);

        job_id
    }

    /// TEE oracle worker updates job status and attaches attestation.
    pub fn complete_job(
        env: Env,
        admin: Address,
        job_id: String,
        cert_id: String,
        attestation: Bytes,
    ) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let mut job: DetectionJob = env
            .storage()
            .persistent()
            .get(&DataKey::Job(job_id.clone()))
            .expect("job not found");

        job.status = 2; // COMPLETE
        job.completed_at = env.ledger().timestamp();
        job.cert_id = cert_id;
        job.attestation = attestation;

        env.storage()
            .persistent()
            .set(&DataKey::Job(job_id), &job);
    }

    /// Mark a job as failed.
    pub fn fail_job(env: Env, admin: Address, job_id: String) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let mut job: DetectionJob = env
            .storage()
            .persistent()
            .get(&DataKey::Job(job_id.clone()))
            .expect("job not found");

        job.status = 3; // FAILED
        env.storage()
            .persistent()
            .set(&DataKey::Job(job_id), &job);
    }

    /// Retrieve a job by ID.
    pub fn get_job(env: Env, job_id: String) -> DetectionJob {
        env.storage()
            .persistent()
            .get(&DataKey::Job(job_id))
            .expect("job not found")
    }

    fn require_admin(env: &Env, caller: &Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("admin"))
            .expect("not initialized");
        assert!(admin == *caller, "unauthorized");
    }
}
