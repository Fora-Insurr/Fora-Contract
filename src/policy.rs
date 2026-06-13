use soroban_sdk::{Address, Env};
use crate::types::{Policy, PolicyStatus, PremiumInput};
use crate::storage;
use crate::premium;

pub fn create_policy(env: &Env, holder: Address, input: PremiumInput, coverage: i128, duration_days: u64) -> u64 {
    let premium = premium::calculate_premium(&input);
    let policy_id = storage::next_policy_id(env);
    let expiry = env.ledger().timestamp() + duration_days * 86400;

    let policy = Policy {
        policy_id,
        holder: holder.clone(),
        premium,
        coverage,
        expiry,
        status: PolicyStatus::Active,
    };

    storage::set_policy(env, &holder, policy_id, &policy);
    policy_id
}

pub fn renew_policy(env: &Env, holder: Address, policy_id: u64, duration_days: u64) {
    let mut policy = storage::get_policy(env, &holder, policy_id).expect("policy not found");
    policy.expiry = env.ledger().timestamp() + duration_days * 86400;
    policy.status = PolicyStatus::Active;
    storage::set_policy(env, &holder, policy_id, &policy);
}

pub fn terminate_policy(env: &Env, holder: Address, policy_id: u64) {
    let mut policy = storage::get_policy(env, &holder, policy_id).expect("policy not found");
    policy.status = PolicyStatus::Terminated;
    storage::set_policy(env, &holder, policy_id, &policy);
}
