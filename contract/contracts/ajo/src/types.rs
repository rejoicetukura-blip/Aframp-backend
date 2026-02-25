use soroban_sdk::{contracttype, Address, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub admin: Address,
    pub token_address: Address,
    pub contribution_amount: i128,
    pub members: Vec<Address>,
    pub total_contributed: i128,
    pub payout_recipient: Option<Address>,
    pub is_active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contribution {
    pub member: Address,
    pub amount: i128,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Payout {
    pub recipient: Address,
    pub amount: i128,
    pub timestamp: u64,
}
