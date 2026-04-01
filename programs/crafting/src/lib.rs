use anchor_lang::prelude::*;
use item_nft::program::ItemNft;
use resource_manager::program::ResourceManager;

declare_id!("GfFyDbWAQK9u1GbhHFWVSPKd7F4ULW8rKxd6xAYRB2tE");

#[program]
pub mod crafting {
    use super::*;

    pub fn craft_item(ctx: Context<CraftItem>, item_type: u8) -> Result<()> {
        let recipe = Recipe::from_item_type(item_type)?;

        burn_one(
            &ctx.accounts.resource_manager_program,
            &ctx.accounts.wood_mint,
            &ctx.accounts.player_wood_token_account,
            &ctx.accounts.authority,
            &ctx.accounts.token_program,
            recipe.wood,
        )?;

        burn_one(
            &ctx.accounts.resource_manager_program,
            &ctx.accounts.iron_mint,
            &ctx.accounts.player_iron_token_account,
            &ctx.accounts.authority,
            &ctx.accounts.token_program,
            recipe.iron,
        )?;

        burn_one(
            &ctx.accounts.resource_manager_program,
            &ctx.accounts.gold_mint,
            &ctx.accounts.player_gold_token_account,
            &ctx.accounts.authority,
            &ctx.accounts.token_program,
            recipe.gold,
        )?;

        burn_one(
            &ctx.accounts.resource_manager_program,
            &ctx.accounts.leather_mint,
            &ctx.accounts.player_leather_token_account,
            &ctx.accounts.authority,
            &ctx.accounts.token_program,
            recipe.leather,
        )?;

        burn_one(
            &ctx.accounts.resource_manager_program,
            &ctx.accounts.stone_mint,
            &ctx.accounts.player_stone_token_account,
            &ctx.accounts.authority,
            &ctx.accounts.token_program,
            recipe.stone,
        )?;

        burn_one(
            &ctx.accounts.resource_manager_program,
            &ctx.accounts.diamond_mint,
            &ctx.accounts.player_diamond_token_account,
            &ctx.accounts.authority,
            &ctx.accounts.token_program,
            recipe.diamond,
        )?;

        let item_cpi_program = ctx.accounts.item_nft_program.to_account_info();

        let item_cpi_accounts = item_nft::cpi::accounts::MintItem {
            item_config: ctx.accounts.item_config.to_account_info(),
            item_metadata: ctx.accounts.item_metadata.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let item_cpi_ctx = CpiContext::new(item_cpi_program, item_cpi_accounts);
        item_nft::cpi::mint_item(item_cpi_ctx, item_type)?;

        Ok(())
    }
}

fn burn_one<'info>(
    resource_manager_program: &Program<'info, ResourceManager>,
    mint: &AccountInfo<'info>,
    player_token_account: &AccountInfo<'info>,
    authority: &Signer<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }

    let cpi_program = resource_manager_program.to_account_info();

    let cpi_accounts = resource_manager::cpi::accounts::BurnResource {
        player: authority.to_account_info(),
        mint: mint.clone(),
        player_token_account: player_token_account.clone(),
        token_program: token_program.clone(),
    };

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    resource_manager::cpi::burn_resource(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(item_type: u8)]
pub struct CraftItem<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK:
    pub wood_mint: AccountInfo<'info>,
    /// CHECK:
    pub iron_mint: AccountInfo<'info>,
    /// CHECK:
    pub gold_mint: AccountInfo<'info>,
    /// CHECK:
    pub leather_mint: AccountInfo<'info>,
    /// CHECK:
    pub stone_mint: AccountInfo<'info>,
    /// CHECK:
    pub diamond_mint: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub player_wood_token_account: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub player_iron_token_account: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub player_gold_token_account: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub player_leather_token_account: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub player_stone_token_account: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub player_diamond_token_account: AccountInfo<'info>,

    /// CHECK:
    pub token_program: AccountInfo<'info>,

    /// CHECK:
    pub item_config: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub item_metadata: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub resource_manager_program: Program<'info, ResourceManager>,
    pub item_nft_program: Program<'info, ItemNft>,
}

pub struct Recipe {
    pub wood: u64,
    pub iron: u64,
    pub gold: u64,
    pub leather: u64,
    pub stone: u64,
    pub diamond: u64,
}

impl Recipe {
    pub fn from_item_type(item_type: u8) -> Result<Self> {
        match item_type {
            0 => Ok(Self {
                wood: 1,
                iron: 3,
                gold: 0,
                leather: 1,
                stone: 0,
                diamond: 0,
            }), // шабля
            1 => Ok(Self {
                wood: 2,
                iron: 1,
                gold: 0,
                leather: 2,
                stone: 0,
                diamond: 0,
            }), // щит
            2 => Ok(Self {
                wood: 0,
                iron: 2,
                gold: 2,
                leather: 0,
                stone: 1,
                diamond: 0,
            }), // булава
            _ => err!(CraftingError::InvalidItemType),
        }
    }
}

#[error_code]
pub enum CraftingError {
    #[msg("Invalid item type")]
    InvalidItemType,
}
