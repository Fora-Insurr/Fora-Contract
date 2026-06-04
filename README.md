
# Nifora-Insur Smart Contracts

> Decentralized insurance protocol built on Stellar Soroban.

## Overview

Nifora-Insur Smart Contracts manage insurance policies, premium calculations, claim submissions, community voting, and automated payouts.

All critical insurance operations execute on-chain, ensuring transparency, auditability, and trustless claim resolution.

## Core Features

### Premium Calculation

Risk-adjusted premium generation using configurable multipliers:

* Age
* Location
* Coverage type
* Safety profile
* Historical risk data

### Policy Lifecycle

* Generate premium
* Create policy
* Renew policy
* Terminate policy

### Claims

* Submit claims
* Attach evidence references
* Track claim status

### Community Governance

Policyholders act as voters.

Supported vote options:

* Approve
* Reject

Majority consensus determines claim outcomes.

### Automated Payouts

Approved claims trigger automatic token transfers directly to policyholders.

## Contract Architecture

```text
┌─────────────────────────┐
│ Premium Calculator      │
└──────────┬──────────────┘
           │
           ▼
┌─────────────────────────┐
│ Policy Contract         │
├─────────────────────────┤
│ Create Policy           │
│ Renew Policy            │
│ Terminate Policy        │
│ Submit Claim            │
│ Vote On Claim           │
└──────────┬──────────────┘
           │
           ▼
┌─────────────────────────┐
│ Token Transfers         │
└─────────────────────────┘
```

## Events

### Policy Events

* PremiumGenerated
* PolicyInitiated
* PolicyRenewed
* PolicyTerminated

### Claim Events

* ClaimFiled
* VoteLogged
* ClaimStatusChanged
* ClaimProcessed

## Project Structure

```text
src/
├── lib.rs
├── types.rs
├── storage.rs
├── premium.rs
├── policy.rs
├── claim.rs
└── token.rs
```

## Storage Design

```text
policies[address][policy_id]

claims[claim_id]

votes[claim_id][address]

voters[]

total_voters

claim_id_counter
```

## Build

```bash
cargo build \
--target wasm32-unknown-unknown \
--release
```

## Test

```bash
cargo test
```

## Deploy

```bash
soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/nifora_insur.wasm
```

## Technology Stack

| Layer              | Technology  |
| ------------------ | ----------- |
| Language           | Rust        |
| Smart Contracts    | Soroban SDK |
| Blockchain         | Stellar     |
| Compilation Target | WASM        |

## Security Considerations

* Access-controlled admin functions
* Double-voting prevention
* Claim status validation
* Secure payout execution
* Immutable audit trail

## License

MIT License
