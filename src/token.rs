use soroban_sdk::{Address, Env};
use soroban_sdk::token::Client as TokenClient;

pub fn transfer(env: &Env, token: &Address, from: &Address, to: &Address, amount: i128) {
    TokenClient::new(env, token).transfer(from, to, &amount);
}
