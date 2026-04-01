use anchor_lang::prelude::*;

declare_id!("Hw9nBuG7VuSRKkXC5X2CVG8ESh7sCTPWG82auUWuk2FH");

#[program]
pub mod magic_token {
    use super::*;

    pub fn initialize_magic_config(ctx: Context<InitializeMagicConfig>) -> Result<()> {
        let config = &mut ctx.accounts.magic_config;
        config.admin = ctx.accounts.admin.key();
        config.bump = ctx.bumps.magic_config;
        Ok(())
    }

    pub fn initialize_player_magic(ctx: Context<InitializePlayerMagic>) -> Result<()> {
        let balance = &mut ctx.accounts.player_magic_balance;
        balance.owner = ctx.accounts.authority.key();
        balance.amount = 0;
        balance.bump = ctx.bumps.player_magic_balance;
        Ok(())
    }

    pub fn mint_magic(ctx: Context<MintMagic>, amount: u64) -> Result<()> {
        require!(amount > 0, MagicError::InvalidAmount);

        let balance = &mut ctx.accounts.player_magic_balance;

        require_keys_eq!(
            balance.owner,
            ctx.accounts.authority.key(),
            MagicError::UnauthorizedOwner
        );

        balance.amount = balance
            .amount
            .checked_add(amount)
            .ok_or(MagicError::Overflow)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMagicConfig<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + MagicConfig::INIT_SPACE,
        seeds = [b"magic_config"],
        bump
    )]
    pub magic_config: Account<'info, MagicConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePlayerMagic<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + PlayerMagicBalance::INIT_SPACE,
        seeds = [b"magic_balance", authority.key().as_ref()],
        bump
    )]
    pub player_magic_balance: Account<'info, PlayerMagicBalance>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintMagic<'info> {
    #[account(
        seeds = [b"magic_config"],
        bump = magic_config.bump
    )]
    pub magic_config: Account<'info, MagicConfig>,

    #[account(
        mut,
        seeds = [b"magic_balance", authority.key().as_ref()],
        bump = player_magic_balance.bump
    )]
    pub player_magic_balance: Account<'info, PlayerMagicBalance>,

    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct MagicConfig {
    pub admin: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct PlayerMagicBalance {
    pub owner: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

#[error_code]
pub enum MagicError {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Overflow")]
    Overflow,
    #[msg("Unauthorized owner")]
    UnauthorizedOwner,
}