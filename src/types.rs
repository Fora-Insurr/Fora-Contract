use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone)]
pub enum PolicyStatus {
    Active,
    Expired,
    Terminated,
}

#[contracttype]
#[derive(Clone)]
pub enum ClaimStatus {
    Pending,
    Approved,
    Rejected,
}

#[contracttype]
#[derive(Clone)]
pub enum VoteOption {
    Approve,
    Reject,
}

#[contracttype]
#[derive(Clone)]
pub struct Policy {
    pub policy_id: u64,
    pub holder: Address,
    pub premium: i128,
    pub coverage: i128,
    pub expiry: u64,
    pub status: PolicyStatus,
}

#[contracttype]
#[derive(Clone)]
pub struct Claim {
    pub claim_id: u64,
    pub policy_id: u64,
    pub holder: Address,
    pub amount: i128,
    pub evidence: String,
    pub status: ClaimStatus,
    pub approve_votes: u32,
    pub reject_votes: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct PremiumInput {
    pub age: u32,
    pub location_risk: u32,
    pub coverage_type: u32,
    pub safety_score: u32,
}
