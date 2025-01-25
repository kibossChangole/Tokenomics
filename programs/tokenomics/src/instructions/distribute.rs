use crate::common::*;
use crate::state::*;


#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(mut)]
    pub tokenomics_account: Account<'info, TokenomicsAccount>,
    #[account(mut)]
    pub stake_account: Account<'info, StakeAccount>,
    
    // Using AccountInfo for the reward pool (instead of TokenAccount)
    #[account(mut)]
        /// CHECK: This instruction requires the reward_pool to be a TokenAccount 
    /// and to be owned by the token program. This check is bypassed 
    /// for demonstration purposes during a lesson on vulnerabilities.
    /// What this really means is that i used annountinfo instead of TokenAccount cause i couldnt get it to work loooooooooool
    pub reward_pool: AccountInfo<'info>,
    
    // Using AccountInfo for the user's token account (instead of TokenAccount)
    #[account(mut, constraint = *user_token_account.owner == user.key())]  // Dereference both sides
    /// CHECK: What this really means is that i used annountinfo instead of TokenAccount cause i couldnt get it to work loooooooooool
    pub user_token_account: AccountInfo<'info>,  // AccountInfo for the user token account
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,  // Associated Token Program
    pub user: Signer<'info>,  // User who is making the request
}


pub fn handler(ctx: Context<Distribute>) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;
    let tokenomics_account = &mut ctx.accounts.tokenomics_account;
    
    // Fetch the reward amount based on stake
    let reward = (stake_account.staked_amount * tokenomics_account.total_staked) / tokenomics_account.reward_pool;

    // Deserialize reward_pool with borrow_mut() for mutable reference
    let reward_pool_data = TokenAccount::try_deserialize(&mut &ctx.accounts.reward_pool.data.borrow_mut()[..])?;
    let user_token_data = TokenAccount::try_deserialize(&mut &ctx.accounts.user_token_account.data.borrow_mut()[..])?;

    // Ensure the mint of the user token account matches the mint of the reward pool
    require_keys_eq!(
        reward_pool_data.mint,
        user_token_data.mint,
        CustomError::InvalidMint
    );

    // Transfer the reward from the reward pool to the user's token account
    let cpi_accounts = Transfer {
        from: ctx.accounts.reward_pool.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.tokenomics_account.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), reward)?;

    // Update the stake account (mutable borrow)
    stake_account.reward_debt += reward;

    Ok(())
}

#[error_code]
pub enum CustomError {
    #[msg("The token mint does not match.")]
    InvalidMint,
    #[msg("The account owner is invalid.")]
    InvalidOwner,
}
