#![cfg(test)]

//! TTL verification tests for governance contract storage entries.
//!
//! Storage layout:
//!  - `persistent()` : Proposal, HasVoted
//!  - `instance()`   : Admin, VotingToken, ProposalCount
//!  - `temporary()`  : token Allowance (votechain-token contract)
//!
//! These tests manipulate `env.ledger().sequence_number` to simulate ledger
//! advancement and use `get_ttl` (Soroban SDK v21+) to assert TTL values.

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String,
};

use crate::{
    storage,
    types::{DataKey, Vote},
    GovernanceContract, GovernanceContractClient,
};

// ── Ledger settings for TTL tests ─────────────────────────────────────────────

const SEQ_START: u32 = 100_000;
const MIN_PERSISTENT_TTL: u32 = 500;
const MIN_TEMP_TTL: u32 = 100;
const MAX_ENTRY_TTL: u32 = 15_000;

fn setup_ttl_env() -> Env {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|l| {
        l.sequence_number = SEQ_START;
        l.timestamp = 0;
        l.min_persistent_entry_ttl = MIN_PERSISTENT_TTL;
        l.min_temp_entry_ttl = MIN_TEMP_TTL;
        l.max_entry_ttl = MAX_ENTRY_TTL;
    });
    env
}

fn setup_contracts(
    env: &Env,
) -> (GovernanceContractClient<'static>, Address, Address, Address) {
    let gov_id = env.register(GovernanceContract, ());
    let client = GovernanceContractClient::new(env, &gov_id);

    let admin = Address::generate(env);

    let tok_id = env.register(votechain_token::TokenContract, ());
    let tok = votechain_token::TokenContractClient::new(env, &tok_id);
    tok.initialize(&admin, &1_000_000);

    client.initialize(&admin, &tok_id);

    (client, gov_id, admin, tok_id)
}

// ── 1. Proposal entry: initial TTL equals min_persistent_entry_ttl - 1 ───────

#[test]
fn test_proposal_initial_ttl() {
    let env = setup_ttl_env();
    let (client, gov_id, _admin, _tok_id) = setup_contracts(&env);

    let proposer = Address::generate(&env);
    let id = client.create_proposal(
        &proposer,
        &String::from_str(&env, "T"),
        &String::from_str(&env, "D"),
        &100,
        &3600,
    );

    // TTL excludes the current ledger, so it is min_persistent_entry_ttl - 1.
    env.as_contract(&gov_id, || {
        let ttl = env.storage().persistent().get_ttl(&DataKey::Proposal(id));
        assert_eq!(ttl, MIN_PERSISTENT_TTL - 1);
    });
}

// ── 2. Proposal entry persists after partial ledger advancement ───────────────

#[test]
fn test_proposal_persists_after_ledger_advance() {
    let env = setup_ttl_env();
    let (client, gov_id, _admin, _tok_id) = setup_contracts(&env);

    let proposer = Address::generate(&env);
    let id = client.create_proposal(
        &proposer,
        &String::from_str(&env, "T"),
        &String::from_str(&env, "D"),
        &100,
        &3600,
    );

    // Advance 400 ledgers — still within the 499-ledger initial TTL.
    env.ledger().with_mut(|l| l.sequence_number += 400);

    env.as_contract(&gov_id, || {
        assert!(env.storage().persistent().has(&DataKey::Proposal(id)));
        let ttl = env.storage().persistent().get_ttl(&DataKey::Proposal(id));
        assert_eq!(ttl, MIN_PERSISTENT_TTL - 1 - 400);
    });
}

// ── 3. HasVoted entry: initial TTL ───────────────────────────────────────────

#[test]
fn test_has_voted_initial_ttl() {
    let env = setup_ttl_env();
    let (client, gov_id, admin, tok_id) = setup_contracts(&env);

    let voter = Address::generate(&env);
    let tok = votechain_token::TokenContractClient::new(&env, &tok_id);
    tok.mint(&admin, &voter, &500_000);

    let id = client.create_proposal(
        &voter,
        &String::from_str(&env, "T"),
        &String::from_str(&env, "D"),
        &100,
        &3600,
    );
    client.cast_vote(&voter, &id, &Vote::Yes);

    env.as_contract(&gov_id, || {
        let ttl = env
            .storage()
            .persistent()
            .get_ttl(&DataKey::HasVoted(id, voter.clone()));
        assert_eq!(ttl, MIN_PERSISTENT_TTL - 1);
    });
}

// ── 4. Instance storage (Admin, VotingToken, ProposalCount): initial TTL ──────

#[test]
fn test_instance_initial_ttl() {
    let env = setup_ttl_env();
    let (_client, gov_id, _admin, _tok_id) = setup_contracts(&env);

    env.as_contract(&gov_id, || {
        // Instance storage shares one TTL for all entries.
        let ttl = env.storage().instance().get_ttl();
        assert_eq!(ttl, MIN_PERSISTENT_TTL - 1);
    });
}

// ── 5. Instance storage remains accessible after ledger advancement ───────────

#[test]
fn test_instance_persists_after_ledger_advance() {
    let env = setup_ttl_env();
    let (client, gov_id, admin, _tok_id) = setup_contracts(&env);

    env.ledger().with_mut(|l| l.sequence_number += 400);

    env.as_contract(&gov_id, || {
        assert!(env.storage().instance().has(&DataKey::Admin));
        assert!(env.storage().instance().has(&DataKey::VotingToken));
        assert_eq!(storage::get_admin(&env).unwrap(), admin);
    });

    assert_eq!(client.proposal_count(), 0);
}

// ── 6. Temporary storage (token allowance) expires after TTL elapses ─────────

#[test]
fn test_temporary_allowance_expires() {
    let env = setup_ttl_env();
    let (_client, _gov_id, admin, tok_id) = setup_contracts(&env);

    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let tok = votechain_token::TokenContractClient::new(&env, &tok_id);

    tok.mint(&admin, &owner, &1_000);
    tok.approve(&owner, &spender, &500);

    // Confirm allowance is set.
    assert_eq!(tok.allowance(&owner, &spender), 500);

    // Advance ledger past min_temp_entry_ttl (TTL = 99 → advance 100 ledgers).
    env.ledger().with_mut(|l| l.sequence_number += MIN_TEMP_TTL + 1);

    // Expired temporary entry reads as 0 (behaves as if deleted).
    assert_eq!(tok.allowance(&owner, &spender), 0);
}

// ── 7. Proposal data remains accessible through the full voting lifecycle ─────

#[test]
fn test_proposal_accessible_through_full_lifecycle() {
    let env = setup_ttl_env();
    let (client, gov_id, admin, tok_id) = setup_contracts(&env);

    let voter = Address::generate(&env);
    let tok = votechain_token::TokenContractClient::new(&env, &tok_id);
    tok.mint(&admin, &voter, &1_000_000);

    let id = client.create_proposal(
        &voter,
        &String::from_str(&env, "Lifecycle"),
        &String::from_str(&env, "desc"),
        &100,
        &200,
    );

    client.cast_vote(&voter, &id, &Vote::Yes);

    env.ledger().with_mut(|l| {
        l.timestamp += 201;
        l.sequence_number += 50;
    });
    client.finalise(&id);
    client.execute(&admin, &id);

    // Persistent entry must still be present.
    env.as_contract(&gov_id, || {
        assert!(env.storage().persistent().has(&DataKey::Proposal(id)));
    });

    let proposal = client.get_proposal(&id);
    assert_eq!(proposal.id, id);
}
