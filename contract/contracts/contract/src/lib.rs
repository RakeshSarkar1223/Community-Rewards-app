#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol, Map};

#[contract]
pub struct CommunityRewards;

#[contractimpl]
impl CommunityRewards {
    // Add reward points to a user (only admin)
    pub fn add_reward(env: Env, admin: Address, user: Address, points: u32) {
        admin.require_auth();

        let mut rewards: Map<Address, u32> =
            env.storage().instance().get(&symbol_short!("REWARDS")).unwrap_or(Map::new(&env));

        let current = rewards.get(user.clone()).unwrap_or(0);
        rewards.set(user, current + points);

        env.storage().instance().set(&symbol_short!("REWARDS"), &rewards);
    }

    // Get reward points of a user
    pub fn get_reward(env: Env, user: Address) -> u32 {
        let rewards: Map<Address, u32> =
            env.storage().instance().get(&symbol_short!("REWARDS")).unwrap_or(Map::new(&env));

        rewards.get(user).unwrap_or(0)
    }
}