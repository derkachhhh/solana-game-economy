use anchor_lang::prelude::*;
use item_nft::program::ItemNft;
use magic_token::program::MagicToken;

declare_id!("GoVuhHUxjGJoe7BH2YMGyeodAPRXHcrr9kfJRsiV5mGh");

#[program]
pub mod marketplace {
    use super::*;

    pub fn redeem_item(ctx: Context<RedeemItem>) -> Result<()> {
        let item_type = ctx.accounts.item_metadata.item_type;
        let reward = reward_for_item(item_type)?;

        require_keys_eq!(
            ctx.accounts.item_metadata.owner,
            ctx.accounts.authority.key(),
            MarketplaceError::NotItemOwner
        );

        let magic_cpi_program = ctx.accounts.magic_token_program.to_account_info();

        let magic_cpi_accounts = magic_token::cpi::accounts::MintMagic {
            magic_config: ctx.accounts.magic_config.to_account_info(),
            player_magic_balance: ctx.accounts.player_magic_balance.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let magic_cpi_ctx = CpiContext::new(magic_cpi_program, magic_cpi_accounts);
        magic_token::cpi::mint_magic(magic_cpi_ctx, reward)?;

        Ok(())
    }
}

fn reward_for_item(item_type: u8) -> Result<u64> {
    match item_type {
        0 => Ok(10), // шабля
        1 => Ok(8),  // щит
        2 => Ok(15), // булава
        _ => err!(MarketplaceError::InvalidItemType),
    }
}

#[derive(Accounts)]
pub struct RedeemItem<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub item_config: Account<'info, item_nft::ItemConfig>,

    #[account(mut)]
    pub item_metadata: Account<'info, item_nft::ItemMetadata>,

    pub magic_config: Account<'info, magic_token::MagicConfig>,

    #[account(mut)]
    pub player_magic_balance: Account<'info, magic_token::PlayerMagicBalance>,

    pub item_nft_program: Program<'info, ItemNft>,
    pub magic_token_program: Program<'info, MagicToken>,
}

#[error_code]
pub enum MarketplaceError {
    #[msg("Invalid item type")]
    InvalidItemType,
    #[msg("You are not the owner of this item")]
    NotItemOwner,
}