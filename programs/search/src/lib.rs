use anchor_lang::prelude::*;
use resource_manager::program::ResourceManager;

declare_id!("HtcJh5rGCvVK5vG1kYcYnjjUgdqXMbLZCohv1BftXggn");

#[program]
pub mod search {
    use super::*;

    pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
        let player = &mut ctx.accounts.player_state;
        player.authority = ctx.accounts.authority.key();
        player.last_search_ts = 0;
        player.bump = ctx.bumps.player_state;
        Ok(())
    }

    pub fn search_resources(ctx: Context<SearchResources>) -> Result<()> {
        let clock = Clock::get()?;
        let player = &mut ctx.accounts.player_state;

        if player.last_search_ts != 0 {
            require!(
                clock.unix_timestamp - player.last_search_ts >= 60,
                SearchError::CooldownActive
            );
        }

        player.last_search_ts = clock.unix_timestamp;

        let cpi_program = ctx.accounts.resource_manager_program.to_account_info();

        let cpi_accounts = resource_manager::cpi::accounts::MintResource {
            game_config: ctx.accounts.game_config.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            player_token_account: ctx.accounts.player_token_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        let random = clock.unix_timestamp.rem_euclid(3);
        let amount = 3;

        if random == 0 {
            resource_manager::cpi::mint_resource(cpi_ctx, amount)?;
        } else if random == 1 {
            resource_manager::cpi::mint_resource(cpi_ctx, amount)?;
        } else {
            resource_manager::cpi::mint_resource(cpi_ctx, amount)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + PlayerState::INIT_SPACE,
        seeds = [b"player", authority.key().as_ref()],
        bump
    )]
    pub player_state: Account<'info, PlayerState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SearchResources<'info> {
    #[account(
        mut,
        seeds = [b"player", authority.key().as_ref()],
        bump = player_state.bump,
        has_one = authority
    )]
    pub player_state: Account<'info, PlayerState>,

    pub authority: Signer<'info>,

    /// CHECK: game_config PDA from resource_manager
    pub game_config: AccountInfo<'info>,

    /// CHECK: chosen resource mint account
    #[account(mut)]
    pub mint: AccountInfo<'info>,

    /// CHECK: player's token account for chosen mint
    #[account(mut)]
    pub player_token_account: AccountInfo<'info>,

    /// CHECK: SPL token program
    pub token_program: AccountInfo<'info>,

    pub resource_manager_program: Program<'info, ResourceManager>,
}

#[account]
#[derive(InitSpace)]
pub struct PlayerState {
    pub authority: Pubkey,
    pub last_search_ts: i64,
    pub bump: u8,
}

#[error_code]
pub enum SearchError {
    #[msg("Search cooldown is still active")]
    CooldownActive,
}