#![no_std]
mod test;

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Bytes, Env, String,
};

/// On-chain detection certificate
#[contracttype]
#[derive(Clone)]
pub struct DetectionCertificate {
    pub content_hash: String,
    pub authenticity_score: u32,   // 0–100
    pub confidence_bps: u32,       // basis points 0–10000 (e.g. 9700 = 0.97)
    pub verdict: u32,              // 0=HUMAN_CREATED 1=AI_GENERATED 2=INCONCLUSIVE
    pub model_version_hash: String,
    pub attestation_proof: Bytes,
    pub timestamp: u64,
    pub creator: Address,
    pub challenge_status: u32,     // 0=NONE 1=PENDING 2=UPHELD 3=OVERTURNED
}

#[contracttype]
pub enum DataKey {
    Certificate(String), // keyed by cert_id
    CertCount,
}

#[contract]
pub struct DetectionContract;

#[contractimpl]
impl DetectionContract {
    /// Mint a new detection certificate. Only callable by the oracle address.
    pub fn mint_certificate(
        env: Env,
        oracle: Address,
        cert_id: String,
        content_hash: String,
        authenticity_score: u32,
        confidence_bps: u32,
        verdict: u32,
        model_version_hash: String,
        attestation_proof: Bytes,
        creator: Address,
    ) -> String {
        oracle.require_auth();

        assert!(authenticity_score <= 100, "score out of range");
        assert!(confidence_bps <= 10_000, "confidence out of range");
        assert!(verdict <= 2, "invalid verdict");

        let timestamp = env.ledger().timestamp();

        let cert = DetectionCertificate {
            content_hash,
            authenticity_score,
            confidence_bps,
            verdict,
            model_version_hash,
            attestation_proof,
            timestamp,
            creator,
            challenge_status: 0,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Certificate(cert_id.clone()), &cert);

        let count: u32 = env
            .storage()
            .instance()
            .get(&symbol_short!("count"))
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&symbol_short!("count"), &(count + 1));

        cert_id
    }

    /// Retrieve a certificate by ID.
    pub fn get_certificate(env: Env, cert_id: String) -> DetectionCertificate {
        env.storage()
            .persistent()
            .get(&DataKey::Certificate(cert_id))
            .expect("certificate not found")
    }

    /// Update challenge status. Only callable by the oracle.
    pub fn set_challenge_status(
        env: Env,
        oracle: Address,
        cert_id: String,
        status: u32,
    ) {
        oracle.require_auth();
        assert!(status <= 3, "invalid challenge status");

        let mut cert: DetectionCertificate = env
            .storage()
            .persistent()
            .get(&DataKey::Certificate(cert_id.clone()))
            .expect("certificate not found");

        cert.challenge_status = status;
        env.storage()
            .persistent()
            .set(&DataKey::Certificate(cert_id), &cert);
    }

    /// Total certificates minted.
    pub fn cert_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&symbol_short!("count"))
            .unwrap_or(0)
    }
}
