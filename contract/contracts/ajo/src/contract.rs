use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec, panic_with_error};
use soroban_token_sdk::TokenClient;

use crate::{storage, events, types::{Group, Contribution, Payout}, errors::AjoError};

#[contract]
pub struct AjoContract;

#[contractimpl]
impl AjoContract {
    /// Create a new savings group with token support
    pub fn create_group(
        env: Env,
        admin: Address,
        name: String,
        token_address: Address,
        contribution_amount: i128,
    ) -> u64 {
        admin.require_auth();

        if contribution_amount <= 0 {
            panic_with_error!(&env, AjoError::InvalidAmount);
        }

        let group_id = storage::get_next_group_id(&env);
        let group = Group {
            id: group_id,
            name,
            admin: admin.clone(),
            token_address: token_address.clone(),
            contribution_amount,
            members: Vec::new(&env),
            total_contributed: 0,
            payout_recipient: None,
            is_active: true,
        };

        storage::save_group(&env, &group);
        events::group_created(&env, group_id, &admin, &token_address);

        group_id
    }

    /// Add a member to the group
    pub fn add_member(env: Env, group_id: u64, admin: Address, member: Address) {
        admin.require_auth();

        let mut group = storage::get_group(&env, group_id)
            .unwrap_or_else(|| panic_with_error!(&env, AjoError::GroupNotFound));

        if group.admin != admin {
            panic_with_error!(&env, AjoError::Unauthorized);
        }

        // Check if already a member
        for existing_member in group.members.iter() {
            if existing_member == member {
                panic_with_error!(&env, AjoError::AlreadyMember);
            }
        }

        group.members.push_back(member.clone());
        storage::save_group(&env, &group);
        events::member_added(&env, group_id, &member);
    }

    /// Contribute to the group with actual token transfer
    pub fn contribute(env: Env, group_id: u64, member: Address, amount: i128) -> bool {
        member.require_auth();

        let mut group = storage::get_group(&env, group_id)
            .unwrap_or_else(|| panic_with_error!(&env, AjoError::GroupNotFound));

        if !group.is_active {
            panic_with_error!(&env, AjoError::ContributionPeriodNotActive);
        }

        if amount <= 0 || amount != group.contribution_amount {
            panic_with_error!(&env, AjoError::InvalidAmount);
        }

        // Verify member is in the group
        let mut is_member = false;
        for group_member in group.members.iter() {
            if group_member == member {
                is_member = true;
                break;
            }
        }
        if !is_member {
            panic_with_error!(&env, AjoError::NotMember);
        }

        // Execute token transfer from member to contract
        let token = TokenClient::new(&env, &group.token_address);
        let contract_address = env.current_contract_address();
        
        // Transfer tokens from member to contract
        token.transfer(&member, &contract_address, &amount);

        // Update group state
        group.total_contributed += amount;
        storage::save_group(&env, &group);

        // Record contribution
        let contribution = Contribution {
            member: member.clone(),
            amount,
            timestamp: env.ledger().timestamp(),
        };
        storage::save_contribution(&env, group_id, &contribution);
        events::contribution_made(&env, group_id, &member, amount);

        true
    }

    /// Execute payout to recipient with actual token transfer
    pub fn execute_payout(
        env: Env,
        group_id: u64,
        admin: Address,
        recipient: Address,
        amount: i128,
    ) -> bool {
        admin.require_auth();

        let mut group = storage::get_group(&env, group_id)
            .unwrap_or_else(|| panic_with_error!(&env, AjoError::GroupNotFound));

        if group.admin != admin {
            panic_with_error!(&env, AjoError::Unauthorized);
        }

        if amount <= 0 {
            panic_with_error!(&env, AjoError::InvalidAmount);
        }

        // Check contract has sufficient balance
        let contract_address = env.current_contract_address();
        let token = TokenClient::new(&env, &group.token_address);
        let contract_balance = token.balance(&contract_address);

        if contract_balance < amount {
            panic_with_error!(&env, AjoError::InsufficientContractBalance);
        }

        // Execute token transfer from contract to recipient
        token.transfer(&contract_address, &recipient, &amount);

        // Update group state
        group.total_contributed -= amount;
        group.payout_recipient = Some(recipient.clone());
        storage::save_group(&env, &group);

        // Record payout
        let payout = Payout {
            recipient: recipient.clone(),
            amount,
            timestamp: env.ledger().timestamp(),
        };
        storage::save_payout(&env, group_id, &payout);
        events::payout_executed(&env, group_id, &recipient, amount);

        true
    }

    /// Check contract balance for a specific token
    pub fn check_balance(env: Env, token_address: Address) -> i128 {
        let contract_address = env.current_contract_address();
        let token = TokenClient::new(&env, &token_address);
        token.balance(&contract_address)
    }

    /// Emergency withdrawal for admin (safety mechanism)
    pub fn emergency_withdraw(
        env: Env,
        group_id: u64,
        admin: Address,
        amount: i128,
    ) -> bool {
        admin.require_auth();

        let group = storage::get_group(&env, group_id)
            .unwrap_or_else(|| panic_with_error!(&env, AjoError::GroupNotFound));

        if group.admin != admin {
            panic_with_error!(&env, AjoError::Unauthorized);
        }

        if amount <= 0 {
            panic_with_error!(&env, AjoError::InvalidAmount);
        }

        let contract_address = env.current_contract_address();
        let token = TokenClient::new(&env, &group.token_address);
        let contract_balance = token.balance(&contract_address);

        if contract_balance < amount {
            panic_with_error!(&env, AjoError::InsufficientContractBalance);
        }

        // Transfer to admin
        token.transfer(&contract_address, &admin, &amount);
        events::emergency_withdrawal(&env, &admin, &group.token_address, amount);

        true
    }

    /// Get group details
    pub fn get_group(env: Env, group_id: u64) -> Option<Group> {
        storage::get_group(&env, group_id)
    }

    /// Get all contributions for a group
    pub fn get_contributions(env: Env, group_id: u64) -> Vec<Contribution> {
        storage::get_contributions(&env, group_id)
    }

    /// Get all payouts for a group
    pub fn get_payouts(env: Env, group_id: u64) -> Vec<Payout> {
        storage::get_payouts(&env, group_id)
    }

    /// Deactivate a group
    pub fn deactivate_group(env: Env, group_id: u64, admin: Address) {
        admin.require_auth();

        let mut group = storage::get_group(&env, group_id)
            .unwrap_or_else(|| panic_with_error!(&env, AjoError::GroupNotFound));

        if group.admin != admin {
            panic_with_error!(&env, AjoError::Unauthorized);
        }

        group.is_active = false;
        storage::save_group(&env, &group);
    }
}
