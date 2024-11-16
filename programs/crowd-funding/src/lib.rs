use anchor_lang::prelude::*;

declare_id!("GqnNEhcmYVAUjKLXSWZLQR7nXSeTgAG56Fw1ADs4DToT");

#[program]
pub mod crowd_funding {
    use super::*;

    pub fn create(ctx : Context<Create>, name : String, description : String) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.description = description;
        campaign.amount = 0;
        campaign.admin  = *ctx.accounts.user.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space = 9000, seeds=[b"CAMPAIGN_DEMO".as_ref(), user.key().as_ref()], bump)]
    pub campaign : Account<'info, Campaign>,
    #[account(mut)]
    pub user : Signer<'info>,
    pub system_program : Program<'info, System>
}

#[account]
pub struct Campaign{
    pub admin : Pubkey,
    pub name : String,
    pub description : String,
    pub amount : u64,
}


