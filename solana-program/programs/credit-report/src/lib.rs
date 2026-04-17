use anchor_lang::prelude::*;

declare_id!("Cr3d1tR3p0rt11111111111111111111111111");

#[program]
pub mod credit_report {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
