use soroban_sdk::{Address, Env, String};
use crate::types::{Claim, ClaimStatus, VoteOption};
use crate::storage;

pub fn submit_claim(env: &Env, holder: Address, policy_id: u64, amount: i128, evidence: String) -> u64 {
    let claim_id = storage::next_claim_id(env);
    let claim = Claim {
        claim_id,
        policy_id,
        holder,
        amount,
        evidence,
        status: ClaimStatus::Pending,
        approve_votes: 0,
        reject_votes: 0,
    };
    storage::set_claim(env, claim_id, &claim);
    claim_id
}

pub fn vote_on_claim(env: &Env, voter: Address, claim_id: u64, vote: VoteOption) {
    assert!(!storage::has_voted(env, claim_id, &voter), "already voted");

    let mut claim = storage::get_claim(env, claim_id).expect("claim not found");
    assert!(matches!(claim.status, ClaimStatus::Pending), "claim not pending");

    match vote {
        VoteOption::Approve => claim.approve_votes += 1,
        VoteOption::Reject => claim.reject_votes += 1,
    }

    storage::set_voted(env, claim_id, &voter);

    let total = storage::get_total_voters(env);
    let majority = total / 2 + 1;

    if claim.approve_votes >= majority {
        claim.status = ClaimStatus::Approved;
    } else if claim.reject_votes >= majority {
        claim.status = ClaimStatus::Rejected;
    }

    storage::set_claim(env, claim_id, &claim);
}
