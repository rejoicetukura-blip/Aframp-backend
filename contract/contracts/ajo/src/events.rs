use soroban_sdk::{Env, Address, symbol_short};

pub fn group_created(env: &Env, group_id: u64, admin: &Address, token: &Address) {
    env.events().publish(
        (symbol_short!("grp_new"), admin.clone()),
        (group_id, token.clone()),
    );
}

pub fn contribution_made(env: &Env, group_id: u64, member: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("contrib"), group_id),
        (member.clone(), amount),
    );
}

pub fn payout_executed(env: &Env, group_id: u64, recipient: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("payout"), group_id),
        (recipient.clone(), amount),
    );
}

pub fn member_added(env: &Env, group_id: u64, member: &Address) {
    env.events().publish(
        (symbol_short!("mbr_add"), group_id),
        member.clone(),
    );
}

pub fn emergency_withdrawal(env: &Env, admin: &Address, token: &Address, amount: i128) {
    env.events().publish(
        (symbol_short!("emerg"), admin.clone()),
        (token.clone(), amount),
    );
}
