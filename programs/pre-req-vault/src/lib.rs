pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2vWNPAB91GLTe46j1Zh18BKB1SK6E4W1tWoKQ6uT4CLi");

#[program]
pub mod pre_req_vault {
    use super::*;

    // Initialize
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.intialize(&ctx.bumps)
    }

    // deposit funds
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    // withdraw funds
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    // close
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}
