#![no_std]

mod types;
mod storage;
mod premium;
mod policy;
mod claim;
mod token;

use soroban_sdk::{contract, contractimpl, Address, Env, String};
use storage::StorageKey;
use types::{Claim, PremiumInput, VoteOption};

#[contract]
pub struct NiforaInsur;

#[contractimpl]
impl NiforaInsur {
    pub fn initialize(env: Env, admin: Address, token: Address, total_voters: u32) {
        env.storage().instance().set(&StorageKey::Admin, &admin);
        env.storage().instance().set(&StorageKey::Token, &token);
        env.storage().instance().set(&StorageKey::TotalVoters, &total_voters);
    }

    pub fn generate_premium(input: PremiumInput) -> i128 {
        premium::calculate_premium(&input)
    }

    pub fn create_policy(env: Env, holder: Address, input: PremiumInput, coverage: i128, duration_days: u64) -> u64 {
        holder.require_auth();
        policy::create_policy(&env, holder, input, coverage, duration_days)
    }

    pub fn renew_policy(env: Env, holder: Address, policy_id: u64, duration_days: u64) {
        holder.require_auth();
        policy::renew_policy(&env, holder, policy_id, duration_days);
    }

    pub fn terminate_policy(env: Env, holder: Address, policy_id: u64) {
        holder.require_auth();
        policy::terminate_policy(&env, holder, policy_id);
    }

    pub fn submit_claim(env: Env, holder: Address, policy_id: u64, amount: i128, evidence: String) -> u64 {
        holder.require_auth();
        claim::submit_claim(&env, holder, policy_id, amount, evidence)
    }

    pub fn vote_on_claim(env: Env, voter: Address, claim_id: u64, vote: VoteOption) {
        voter.require_auth();
        claim::vote_on_claim(&env, voter, claim_id, vote);
    }

    pub fn process_payout(env: Env, claim_id: u64) {
        let c: Claim = storage::get_claim(&env, claim_id).expect("claim not found");
        assert!(matches!(c.status, types::ClaimStatus::Approved), "claim not approved");
        let token = storage::get_token(&env);
        let admin = storage::get_admin(&env);
        token::transfer(&env, &token, &admin, &c.holder, c.amount);
    }
}
