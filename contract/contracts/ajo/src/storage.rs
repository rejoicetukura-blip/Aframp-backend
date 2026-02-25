use soroban_sdk::{symbol_short, Address, Env, Map, Symbol, Vec};
use crate::types::{Group, Contribution, Payout};

const GROUPS_KEY: Symbol = symbol_short!("GROUPS");
const GROUP_COUNTER: Symbol = symbol_short!("GRP_CNT");
const CONTRIBUTIONS_KEY: Symbol = symbol_short!("CONTRIB");
const PAYOUTS_KEY: Symbol = symbol_short!("PAYOUTS");

pub fn get_next_group_id(env: &Env) -> u64 {
    let current: u64 = env.storage()
        .instance()
        .get(&GROUP_COUNTER)
        .unwrap_or(0);
    env.storage().instance().set(&GROUP_COUNTER, &(current + 1));
    current + 1
}

pub fn save_group(env: &Env, group: &Group) {
    let mut groups = get_all_groups(env);
    groups.set(group.id, group.clone());
    env.storage().instance().set(&GROUPS_KEY, &groups);
}

pub fn get_group(env: &Env, group_id: u64) -> Option<Group> {
    let groups = get_all_groups(env);
    groups.get(group_id)
}

pub fn get_all_groups(env: &Env) -> Map<u64, Group> {
    env.storage()
        .instance()
        .get(&GROUPS_KEY)
        .unwrap_or(Map::new(env))
}

pub fn save_contribution(env: &Env, group_id: u64, contribution: &Contribution) {
    let key = (CONTRIBUTIONS_KEY, group_id);
    let mut contributions: Vec<Contribution> = env.storage()
        .instance()
        .get(&key)
        .unwrap_or(Vec::new(env));
    contributions.push_back(contribution.clone());
    env.storage().instance().set(&key, &contributions);
}

pub fn get_contributions(env: &Env, group_id: u64) -> Vec<Contribution> {
    let key = (CONTRIBUTIONS_KEY, group_id);
    env.storage()
        .instance()
        .get(&key)
        .unwrap_or(Vec::new(env))
}

pub fn save_payout(env: &Env, group_id: u64, payout: &Payout) {
    let key = (PAYOUTS_KEY, group_id);
    let mut payouts: Vec<Payout> = env.storage()
        .instance()
        .get(&key)
        .unwrap_or(Vec::new(env));
    payouts.push_back(payout.clone());
    env.storage().instance().set(&key, &payouts);
}

pub fn get_payouts(env: &Env, group_id: u64) -> Vec<Payout> {
    let key = (PAYOUTS_KEY, group_id);
    env.storage()
        .instance()
        .get(&key)
        .unwrap_or(Vec::new(env))
}
