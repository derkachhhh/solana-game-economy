use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo};

declare_id!("A8mtuKk3f8u97UbKUMzYCPsKajQShBxCYE5RbvDYPheX");

#[program]
pub mod resource_manager {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        let game_config = &mut ctx.accounts.game_config;
        game_config.admin = ctx.accounts.admin.key();
        game_config.bump = ctx.bumps.game_config;

        game_config.wood_mint = Pubkey::default();
        game_config.iron_mint = Pubkey::default();
        game_config.gold_mint = Pubkey::default();
        game_config.leather_mint = Pubkey::default();
        game_config.stone_mint = Pubkey::default();
        game_config.diamond_mint = Pubkey::default();

        Ok(())
    }

    pub fn set_resource_mint(
        ctx: Context<SetResourceMint>,
        resource_type: u8,
        mint: Pubkey,
    ) -> Result<()> {
        let game_config = &mut ctx.accounts.game_config;

        require_keys_eq!(
            game_config.admin,
            ctx.accounts.admin.key(),
            GameError::Unauthorized
        );

        match resource_type {
            0 => game_config.wood_mint = mint,
            1 => game_config.iron_mint = mint,
            2 => game_config.gold_mint = mint,
            3 => game_config.leather_mint = mint,
            4 => game_config.stone_mint = mint,
            5 => game_config.diamond_mint = mint,
            _ => return err!(GameError::InvalidResourceType),
        }

        Ok(())
    }

    pub fn mint_resource(ctx: Context<MintResource>, amount: u64) -> Result<()> {
        require!(amount > 0, GameError::InvalidAmount);

        let game_config = &ctx.accounts.game_config;
        let mint_key = ctx.accounts.mint.key();

        require!(
            mint_key == game_config.wood_mint
                || mint_key == game_config.iron_mint
                || mint_key == game_config.gold_mint
                || mint_key == game_config.leather_mint
                || mint_key == game_config.stone_mint
                || mint_key == game_config.diamond_mint,
            GameError::UnknownMint
        );

        let signer_seeds: &[&[&[u8]]] = &[&[b"game_config", &[game_config.bump]]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.player_token_account.to_account_info(),
            authority: ctx.accounts.game_config.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn burn_resource(ctx: Context<BurnResource>, amount: u64) -> Result<()> {
        require!(amount > 0, GameError::InvalidAmount);

        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.player_token_account.to_account_info(),
            authority: ctx.accounts.player.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );

        token::burn(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + GameConfig::INIT_SPACE,
        seeds = [b"game_config"],
        bump
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetResourceMint<'info> {
    #[account(
        mut,
        seeds = [b"game_config"],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,

    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct MintResource<'info> {
    #[account(
        seeds = [b"game_config"],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,

    /// CHECK:
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub player_token_account: AccountInfo<'info>,

    /// CHECK:
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BurnResource<'info> {
    pub player: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub player_token_account: AccountInfo<'info>,

    /// CHECK:
    pub token_program: AccountInfo<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct GameConfig {
    pub admin: Pubkey,
    pub bump: u8,

    pub wood_mint: Pubkey,
    pub iron_mint: Pubkey,
    pub gold_mint: Pubkey,
    pub leather_mint: Pubkey,
    pub stone_mint: Pubkey,
    pub diamond_mint: Pubkey,
}

#[error_code]
pub enum GameError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid resource type")]
    InvalidResourceType,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Unknown mint")]
    UnknownMint,
}