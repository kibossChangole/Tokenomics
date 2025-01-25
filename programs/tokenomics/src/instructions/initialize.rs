use crate::common::*;
use crate::state::*;


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 8 + 8 + 8 + 1,  seeds = [b"tokenomics_account"] , bump)] // Add seeds
    pub tokenomics_account: Account<'info, TokenomicsAccount>,
    #[account(mut)]
    /// CHECK Uses Account Info which is unsafe, switch to TokenAccount later
    pub authority: Signer<'info>,
    #[account(mut)]
    /// CHECK Uses Account Info which is unsafe, switch to TokenAccount later
    pub fee_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<Initialize>, fee_rate: u64) -> Result<()> {
    let tokenomics = &mut ctx.accounts.tokenomics_account;
    tokenomics.authority = *ctx.accounts.authority.key;
    tokenomics.fee_account = *ctx.accounts.fee_account.key;
    tokenomics.fee_rate = fee_rate;
    tokenomics.total_staked = 0;
    tokenomics.reward_pool = 0;
    // tokenomics.bump = *ctx.bumps.get("tokenomics_account").unwrap(); // No longer needed
    Ok(())

}