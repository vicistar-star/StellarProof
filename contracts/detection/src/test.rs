#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Bytes, Env, String};

use crate::{DetectionContract, DetectionContractClient};

#[test]
fn test_mint_and_get_certificate() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(DetectionContract, ());
    let client = DetectionContractClient::new(&env, &contract_id);

    let oracle = Address::generate(&env);
    let creator = Address::generate(&env);

    let cert_id = client.mint_certificate(
        &oracle,
        &String::from_str(&env, "cert-001"),
        &String::from_str(&env, "sha256:abc123"),
        &85,
        &9700,
        &0, // HUMAN_CREATED
        &String::from_str(&env, "sha256:modelv1"),
        &Bytes::from_slice(&env, b"attestation-proof"),
        &creator,
    );

    assert_eq!(cert_id, String::from_str(&env, "cert-001"));

    let cert = client.get_certificate(&String::from_str(&env, "cert-001"));
    assert_eq!(cert.authenticity_score, 85);
    assert_eq!(cert.verdict, 0);
    assert_eq!(cert.challenge_status, 0);
    assert_eq!(client.cert_count(), 1);
}

#[test]
fn test_set_challenge_status() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(DetectionContract, ());
    let client = DetectionContractClient::new(&env, &contract_id);

    let oracle = Address::generate(&env);
    let creator = Address::generate(&env);

    client.mint_certificate(
        &oracle,
        &String::from_str(&env, "cert-002"),
        &String::from_str(&env, "sha256:def456"),
        &30,
        &8800,
        &1, // AI_GENERATED
        &String::from_str(&env, "sha256:modelv1"),
        &Bytes::from_slice(&env, b"proof"),
        &creator,
    );

    client.set_challenge_status(&oracle, &String::from_str(&env, "cert-002"), &1);

    let cert = client.get_certificate(&String::from_str(&env, "cert-002"));
    assert_eq!(cert.challenge_status, 1); // PENDING
}
