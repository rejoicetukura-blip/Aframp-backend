use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AjoError {
    Unauthorized = 1,
    InsufficientBalance = 2,
    InvalidAmount = 3,
    GroupNotFound = 4,
    AlreadyMember = 5,
    NotMember = 6,
    ContributionPeriodNotActive = 7,
    PayoutNotReady = 8,
    TokenTransferFailed = 9,
    InsufficientContractBalance = 10,
    InvalidTokenAddress = 11,
    AllowanceInsufficient = 12,
}
