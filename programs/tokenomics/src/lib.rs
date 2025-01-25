mod common;
pub mod state;
pub mod instructions;

use crate::instructions::initialize::Initialize;
use crate::instructions::stake::Stake;
use crate::instructions::distribute::Distribute;


use crate::instructions::initialize::__client_accounts_initialize;

use crate::instructions::stake::__client_accounts_stake;
use crate::instructions::distribute::__client_accounts_distribute;

use common::*;

declare_id!("6jSCfzMbiNLv31AcgnPdW1HZcGd3CRchtSS3DBn7GPqF");

#[program]
pub mod tokenomics {
    use super::*;

    
    pub fn initialize(ctx: Context<Initialize>, fee_rate: u64) -> Result<()> {
        instructions::initialize::handler(ctx, fee_rate) // Call handler from initialize module
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake::handler(ctx, amount) // Call handler from stake module
    }

    pub fn distribute(ctx: Context<Distribute>) -> Result<()> {
        instructions::distribute::handler(ctx) // Call handler from distribute module
    }
}
