use crate::common::*;

#[account]
pub struct TokenomicsAccount {
    pub authority: Pubkey,
    pub fee_account: Pubkey,
    pub fee_rate: u64,
    pub total_staked: u64,
    pub reward_pool: u64,
    pub bump: u8,
    pub token_mint: Pubkey,
}

#[account]
pub struct Testaccount {
    pub authority: Pubkey,
}

#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub staked_amount: u64,
    pub reward_debt: u64,
    pub bump: u8,
}