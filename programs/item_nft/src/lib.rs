use anchor_lang::prelude::*;

declare_id!("Fe4iTXT1Ucp254dCfaF3gyWPa2zq2AGwRmijJNuvvPEo");

#[program]
pub mod item_nft {
    use super::*;

    pub fn initialize_item_config(ctx: Context<InitializeItemConfig>) -> Result<()> {
        let config = &mut ctx.accounts.item_config;
        config.admin = ctx.accounts.admin.key();
        config.bump = ctx.bumps.item_config;
        Ok(())
    }

    pub fn mint_item(ctx: Context<MintItem>, item_type: u8) -> Result<()> {
        let item = &mut ctx.accounts.item_metadata;
        item.owner = ctx.accounts.authority.key();
        item.item_type = item_type;
        item.created_at = Clock::get()?.unix_timestamp;
        item.bump = ctx.bumps.item_metadata;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeItemConfig<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + ItemConfig::INIT_SPACE,
        seeds = [b"item_config"],
        bump
    )]
    pub item_config: Account<'info, ItemConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(item_type: u8)]
pub struct MintItem<'info> {
    #[account(
        seeds = [b"item_config"],
        bump = item_config.bump
    )]
    pub item_config: Account<'info, ItemConfig>,

    #[account(
        init,
        payer = authority,
        space = 8 + ItemMetadata::INIT_SPACE,
        seeds = [b"item", authority.key().as_ref(), &[item_type]],
        bump
    )]
    pub item_metadata: Account<'info, ItemMetadata>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct ItemConfig {
    pub admin: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ItemMetadata {
    pub owner: Pubkey,
    pub item_type: u8,
    pub created_at: i64,
    pub bump: u8,
}
