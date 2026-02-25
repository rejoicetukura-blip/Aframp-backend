# Ajo Savings Group Contract

A Stellar smart contract for managing rotating savings and credit associations (Ajo/ROSCA) with full token transfer support.

## Features

- ✅ Create savings groups with custom token support (XLM, USDC, custom tokens)
- ✅ Add members to groups
- ✅ Contribute with actual on-chain token transfers
- ✅ Execute payouts with token transfers
- ✅ Balance checking for contract holdings
- ✅ Emergency withdrawal mechanism for admins
- ✅ Support for multiple token types per group
- ✅ Comprehensive error handling
- ✅ Event emission for all major actions

## Token Transfer Implementation

### Contribute Function
- Validates member authorization
- Checks contribution amount matches group requirement
- Executes `token.transfer(member, contract, amount)`
- Updates group state and records contribution
- Emits contribution event

### Execute Payout Function
- Validates admin authorization
- Checks contract has sufficient balance
- Executes `token.transfer(contract, recipient, amount)`
- Updates group state and records payout
- Emits payout event

### Emergency Withdrawal
- Admin-only function for safety
- Transfers tokens from contract to admin
- Useful for recovering stuck funds or handling emergencies

## Usage

### Create a Group
```rust
let group_id = contract.create_group(
    admin_address,
    "My Savings Group",
    token_contract_address,
    1000 // contribution amount
);
```

### Add Members
```rust
contract.add_member(group_id, admin_address, member_address);
```

### Contribute
```rust
// Member must have approved the contract to spend tokens
contract.contribute(group_id, member_address, 1000);
```

### Execute Payout
```rust
contract.execute_payout(
    group_id,
    admin_address,
    recipient_address,
    5000 // payout amount
);
```

### Check Balance
```rust
let balance = contract.check_balance(token_address);
```

### Emergency Withdrawal
```rust
contract.emergency_withdraw(group_id, admin_address, amount);
```

## Building

```bash
make build
```

## Testing

```bash
make test
```

## Error Codes

- `Unauthorized (1)` - Caller not authorized
- `InsufficientBalance (2)` - Not enough balance
- `InvalidAmount (3)` - Amount is zero or negative
- `GroupNotFound (4)` - Group doesn't exist
- `AlreadyMember (5)` - Member already in group
- `NotMember (6)` - Address is not a group member
- `ContributionPeriodNotActive (7)` - Group is inactive
- `PayoutNotReady (8)` - Payout conditions not met
- `TokenTransferFailed (9)` - Token transfer failed
- `InsufficientContractBalance (10)` - Contract doesn't have enough tokens
- `InvalidTokenAddress (11)` - Invalid token contract
- `AllowanceInsufficient (12)` - Token allowance too low

## Security Considerations

- All state-changing functions require proper authorization
- Token transfers are atomic - they either succeed completely or revert
- Balance checks prevent overdrafts
- Emergency withdrawal provides admin escape hatch
- Group deactivation prevents new contributions

## Gas Optimization

- Efficient storage using Maps and Vecs
- Minimal state updates
- Direct token client calls without unnecessary wrapping
