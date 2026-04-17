use anchor_lang::prelude::*;

declare_id!("Cr3d1tR3p0rt11111111111111111111111111");

#[program]
pub mod credit_report {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn register_enterprise(
        ctx: Context<RegisterEnterprise>,
        tee_pubkey: [u8; 64],
    ) -> Result<()> {
        let enterprise = &mut ctx.accounts.enterprise;
        enterprise.owner = ctx.accounts.owner.key();
        enterprise.tee_pubkey = tee_pubkey;
        enterprise.bump = ctx.bumps.enterprise;
        Ok(())
    }

    pub fn submit_report(
        ctx: Context<SubmitReport>,
        version: u8,
        report_hash: [u8; 32],
        credit_score: u32,
        risk_level: u8,
        signature: [u8; 64],
    ) -> Result<()> {
        let report = &mut ctx.accounts.report;
        report.version = version;
        report.enterprise = ctx.accounts.enterprise.key();
        report.report_hash = report_hash;
        report.credit_score = credit_score;
        report.risk_level = risk_level;
        report.tee_pubkey = ctx.accounts.enterprise.tee_pubkey;
        report.signature = signature;
        report.timestamp = Clock::get()?.unix_timestamp;
        report.verified = false;
        report.bump = ctx.bumps.report;
        
        msg!("Report submitted: {:?}", report_hash);
        Ok(())
    }

    pub fn verify_report(ctx: Context<VerifyReport>) -> Result<()> {
        let report = &mut ctx.accounts.report;
        
        // TODO: ECDSA signature verification
        // For now, mark as verified for testing
        report.verified = true;
        
        msg!("Report verified");
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct EnterpriseAccount {
    pub owner: Pubkey,
    pub tee_pubkey: [u8; 64],
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct CreditReportAccount {
    pub version: u8,
    pub enterprise: Pubkey,
    pub report_hash: [u8; 32],
    pub credit_score: u32,
    pub risk_level: u8,
    pub tee_pubkey: [u8; 64],
    pub signature: [u8; 64],
    pub timestamp: i64,
    pub verified: bool,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(tee_pubkey: [u8; 64])]
pub struct RegisterEnterprise<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + EnterpriseAccount::INIT_SPACE,
        seeds = [b"enterprise", owner.key().as_ref()],
        bump
    )]
    pub enterprise: Account<'info, EnterpriseAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitReport<'info> {
    #[account(mut)]
    pub enterprise: Account<'info, EnterpriseAccount>,
    #[account(
        init,
        payer = enterprise,
        space = 8 + CreditReportAccount::INIT_SPACE,
        seeds = [b"report", enterprise.key().as_ref()],
        bump
    )]
    pub report: Account<'info, CreditReportAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyReport<'info> {
    #[account(mut)]
    pub report: Account<'info, CreditReportAccount>,
}
