#![no_std]

//! Registry contract — governs approved TEE code hashes and AI model versions.
//! Any model update must be registered here before the oracle will accept it.

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String,
};

#[contracttype]
#[derive(Clone)]
pub struct ModelEntry {
    pub model_hash: String,   // sha256 of the model weights
    pub tee_code_hash: String, // PCR0 measurement of the Nitro Enclave image
    pub version_tag: String,  // human-readable e.g. "v1.2.0"
    pub registered_at: u64,
    pub active: bool,
}

#[contracttype]
pub enum DataKey {
    Model(String), // keyed by model_hash
    Admin,
}

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    /// Initialize with a governance admin address.
    pub fn initialize(env: Env, admin: Address) {
        assert!(
            !env.storage().instance().has(&symbol_short!("admin")),
            "already initialized"
        );
        env.storage()
            .instance()
            .set(&symbol_short!("admin"), &admin);
    }

    /// Register a new approved model + TEE code hash pair.
    pub fn register_model(
        env: Env,
        admin: Address,
        model_hash: String,
        tee_code_hash: String,
        version_tag: String,
    ) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let entry = ModelEntry {
            model_hash: model_hash.clone(),
            tee_code_hash,
            version_tag,
            registered_at: env.ledger().timestamp(),
            active: true,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Model(model_hash), &entry);
    }

    /// Deactivate a model (e.g. after a vulnerability is found).
    pub fn deactivate_model(env: Env, admin: Address, model_hash: String) {
        admin.require_auth();
        Self::require_admin(&env, &admin);

        let mut entry: ModelEntry = env
            .storage()
            .persistent()
            .get(&DataKey::Model(model_hash.clone()))
            .expect("model not found");

        entry.active = false;
        env.storage()
            .persistent()
            .set(&DataKey::Model(model_hash), &entry);
    }

    /// Check whether a model hash is currently approved and active.
    pub fn is_approved(env: Env, model_hash: String) -> bool {
        env.storage()
            .persistent()
            .get::<DataKey, ModelEntry>(&DataKey::Model(model_hash))
            .map(|e| e.active)
            .unwrap_or(false)
    }

    /// Retrieve full model entry.
    pub fn get_model(env: Env, model_hash: String) -> ModelEntry {
        env.storage()
            .persistent()
            .get(&DataKey::Model(model_hash))
            .expect("model not found")
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
