#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, Address, Env, String,
};

fn create_token_contract<'a>(env: &Env, admin: &Address) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let contract_address = env.register_stellar_asset_contract(admin.clone());
    (
        token::Client::new(env, &contract_address),
        token::StellarAssetClient::new(env, &contract_address),
    )
}

#[test]
fn test_create_group() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (token_client, _) = create_token_contract(&env, &admin);
    
    let contract_id = env.register_contract(None, AjoContract);
    let client = AjoContractClient::new(&env, &contract_id);

    let group_id = client.create_group(
        &admin,
        &String::from_str(&env, "Test Group"),
        &token_client.address,
        &1000,
    );

    assert_eq!(group_id, 1);

    let group = client.get_group(&group_id).unwrap();
    assert_eq!(group.admin, admin);
    assert_eq!(group.contribution_amount, 1000);
    assert_eq!(group.is_active, true);
}

#[test]
fn test_add_member() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let member = Address::generate(&env);
    let (token_client, _) = create_token_contract(&env, &admin);
    
    let contract_id = env.register_contract(None, AjoContract);
    let client = AjoContractClient::new(&env, &contract_id);

    let group_id = client.create_group(
        &admin,
        &String::from_str(&env, "Test Group"),
        &token_client.address,
        &1000,
    );

    client.add_member(&group_id, &admin, &member);

    let group = client.get_group(&group_id).unwrap();
    assert_eq!(group.members.len(), 1);
    assert_eq!(group.members.get(0).unwrap(), member);
}

#[test]
fn test_contribute_with_token_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let member = Address::generate(&env);
    let (token_client, token_admin) = create_token_contract(&env, &admin);
    
    let contract_id = env.register_contract(None, AjoContract);
    let client = AjoContractClient::new(&env, &contract_id);

    // Mint tokens to member
    token_admin.mint(&member, &5000);

    let group_id = client.create_group(
        &admin,
        &String::from_str(&env, "Test Group"),
        &token_client.address,
        &1000,
    );

    client.add_member(&group_id, &admin, &member);

    // Contribute
    let result = client.contribute(&group_id, &member, &1000);
    assert_eq!(result, true);

    // Verify balances
    let member_balance = token_client.balance(&member);
    assert_eq!(member_balance, 4000);

    let contract_balance = token_client.balance(&contract_id);
    assert_eq!(contract_balance, 1000);

    // Verify group state
    let group = client.get_group(&group_id).unwrap();
    assert_eq!(group.total_contributed, 1000);

    // Verify contribution recorded
    let contributions = client.get_contributions(&group_id);
    assert_eq!(contributions.len(), 1);
    assert_eq!(contributions.get(0).unwrap().amount, 1000);
}

#[test]
fn test_execute_payout_with_token_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let member = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (token_client, token_admin) = create_token_contract(&env, &admin);
    
    let contract_id = env.register_contract(None, AjoContract);
    let client = AjoContractClient::new(&env, &contract_id);

    // Mint tokens to member
    token_admin.mint(&member, &5000);

    let group_id = client.create_group(
        &admin,
        &String::from_str(&env, "Test Group"),
        &token_client.address,
        &1000,
    );

    client.add_member(&group_id, &admin, &member);
    client.contribute(&group_id, &member, &1000);

    // Execute payout
    let result = client.execute_payout(&group_id, &admin, &recipient, &800);
    assert_eq!(result, true);

    // Verify balances
    let recipient_balance = token_client.balance(&recipient);
    assert_eq!(recipient_balance, 800);

    let contract_balance = token_client.balance(&contract_id);
    assert_eq!(contract_balance, 200);

    // Verify payout recorded
    let payouts = client.get_payouts(&group_id);
    assert_eq!(payouts.len(), 1);
    assert_eq!(payouts.get(0).unwrap().amount, 800);
}

#[test]
#[should_panic(expected = "InsufficientContractBalance")]
fn test_payout_fails_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    let (token_client, _) = create_token_contract(&env, &admin);
    
    let contract_id = env.register_contract(None, AjoContract);
    let client = AjoContractClient::new(&env, &contract_id);

    let group_id = client.create_group(
        &admin,
        &String::from_str(&env, "Test Group"),
        &token_client.address,
        &1000,
    );

    // Try to payout without any contributions
    client.execute_payout(&group_id, &admin, &recipient, &1000);
}

#[test]
fn test_check_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let member = Address::generate(&env);
    let (token_client, token_admin) = create_token_contract(&env, &admin);
    
    let contract_id = env.register_contract(None, AjoContract);
    let client = AjoContractClient::new(&env, &contract_id);

    token_admin.mint(&member, &5000);

    let group_id = client.create_group(
        &admin,
        &String::from_str(&env, "Test Group"),
        &token_client.address,
        &1000,
    );

    client.add_member(&group_id, &admin, &member);
    client.contribute(&group_id, &member, &1000);

    let balance = client.check_balance(&token_client.address);
    assert_eq!(balance, 1000);
}

#[test]
fn test_emergency_withdraw() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let member = Address::generate(&env);
    let (token_client, token_admin) = create_token_contract(&env, &admin);
    
    let contract_id = env.register_contract(None, AjoContract);
    let client = AjoContractClient::new(&env, &contract_id);

    token_admin.mint(&member, &5000);

    let group_id = client.create_group(
        &admin,
        &String::from_str(&env, "Test Group"),
        &token_client.address,
        &1000,
    );

    client.add_member(&group_id, &admin, &member);
    client.contribute(&group_id, &member, &1000);

    // Emergency withdraw
    let result = client.emergency_withdraw(&group_id, &admin, &500);
    assert_eq!(result, true);

    let admin_balance = token_client.balance(&admin);
    assert_eq!(admin_balance, 500);

    let contract_balance = token_client.balance(&contract_id);
    assert_eq!(contract_balance, 500);
}

#[test]
fn test_multiple_contributions() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let member1 = Address::generate(&env);
    let member2 = Address::generate(&env);
    let (token_client, token_admin) = create_token_contract(&env, &admin);
    
    let contract_id = env.register_contract(None, AjoContract);
    let client = AjoContractClient::new(&env, &contract_id);

    token_admin.mint(&member1, &5000);
    token_admin.mint(&member2, &5000);

    let group_id = client.create_group(
        &admin,
        &String::from_str(&env, "Test Group"),
        &token_client.address,
        &1000,
    );

    client.add_member(&group_id, &admin, &member1);
    client.add_member(&group_id, &admin, &member2);

    client.contribute(&group_id, &member1, &1000);
    client.contribute(&group_id, &member2, &1000);

    let contract_balance = token_client.balance(&contract_id);
    assert_eq!(contract_balance, 2000);

    let group = client.get_group(&group_id).unwrap();
    assert_eq!(group.total_contributed, 2000);

    let contributions = client.get_contributions(&group_id);
    assert_eq!(contributions.len(), 2);
}

#[test]
fn test_deactivate_group() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let (token_client, _) = create_token_contract(&env, &admin);
    
    let contract_id = env.register_contract(None, AjoContract);
    let client = AjoContractClient::new(&env, &contract_id);

    let group_id = client.create_group(
        &admin,
        &String::from_str(&env, "Test Group"),
        &token_client.address,
        &1000,
    );

    client.deactivate_group(&group_id, &admin);

    let group = client.get_group(&group_id).unwrap();
    assert_eq!(group.is_active, false);
}

#[test]
#[should_panic(expected = "ContributionPeriodNotActive")]
fn test_contribute_fails_when_inactive() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let member = Address::generate(&env);
    let (token_client, token_admin) = create_token_contract(&env, &admin);
    
    let contract_id = env.register_contract(None, AjoContract);
    let client = AjoContractClient::new(&env, &contract_id);

    token_admin.mint(&member, &5000);

    let group_id = client.create_group(
        &admin,
        &String::from_str(&env, "Test Group"),
        &token_client.address,
        &1000,
    );

    client.add_member(&group_id, &admin, &member);
    client.deactivate_group(&group_id, &admin);

    // Should fail
    client.contribute(&group_id, &member, &1000);
}
