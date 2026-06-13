use soroban_sdk::{contracttype, Address, Env};
use crate::types::{Claim, Policy};

#[contracttype]
pub enum StorageKey {
    Policy(Address, u64),
    Claim(u64),
    Vote(u64, Address),
    Voters,
    TotalVoters,
    ClaimCounter,
    PolicyCounter,
    Admin,
    Token,
}

pub fn get_policy(env: &Env, holder: &Address, policy_id: u64) -> Option<Policy> {
    env.storage().persistent().get(&StorageKey::Policy(holder.clone(), policy_id))
}

pub fn set_policy(env: &Env, holder: &Address, policy_id: u64, policy: &Policy) {
    env.storage().persistent().set(&StorageKey::Policy(holder.clone(), policy_id), policy);
}

pub fn get_claim(env: &Env, claim_id: u64) -> Option<Claim> {
    env.storage().persistent().get(&StorageKey::Claim(claim_id))
}

pub fn set_claim(env: &Env, claim_id: u64, claim: &Claim) {
    env.storage().persistent().set(&StorageKey::Claim(claim_id), claim);
}

pub fn has_voted(env: &Env, claim_id: u64, voter: &Address) -> bool {
    env.storage().persistent().has(&StorageKey::Vote(claim_id, voter.clone()))
}

pub fn set_voted(env: &Env, claim_id: u64, voter: &Address) {
    env.storage().persistent().set(&StorageKey::Vote(claim_id, voter.clone()), &true);
}

pub fn next_claim_id(env: &Env) -> u64 {
    let id: u64 = env.storage().instance().get(&StorageKey::ClaimCounter).unwrap_or(0);
    env.storage().instance().set(&StorageKey::ClaimCounter, &(id + 1));
    id + 1
}

pub fn next_policy_id(env: &Env) -> u64 {
    let id: u64 = env.storage().instance().get(&StorageKey::PolicyCounter).unwrap_or(0);
    env.storage().instance().set(&StorageKey::PolicyCounter, &(id + 1));
    id + 1
}

pub fn get_total_voters(env: &Env) -> u32 {
    env.storage().instance().get(&StorageKey::TotalVoters).unwrap_or(0)
}

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&StorageKey::Admin).unwrap()
}

pub fn get_token(env: &Env) -> Address {
    env.storage().instance().get(&StorageKey::Token).unwrap()
}
