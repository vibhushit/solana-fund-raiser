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

    pub fn withdraw(ctx : Context<Withdraw>, amount : u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;

        if campaign.admin != *user.key{
            return Err(ProgramError::IncorrectProgramId.into());
        }

        let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
        if **campaign.to_account_info().lamports.borrow() - rent_balance < amount {
            return Err(ProgramError::InsufficientFunds.into());
        }

        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

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

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub campaign : Account<'info, Campaign>,
    #[account(mut)]
    pub user : Signer<'info>
}

#[account]
pub struct Campaign{
    pub admin : Pubkey,
    pub name : String,
    pub description : String,
    pub amount : u64,
}


