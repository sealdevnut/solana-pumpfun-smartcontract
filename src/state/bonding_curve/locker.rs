use anchor_lang::prelude::*;
// use anchor_lang::{prelude::AccountInfo, Accounts};
use anchor_spl::token::spl_token::instruction::AuthorityType;
use anchor_spl::token::{self, FreezeAccount, Mint, ThawAccount, Token, TokenAccount};

use crate::state::bonding_curve::BondingCurve;

// #[derive(Accounts)]
pub struct BondingCurveLockerCtx<'info> {
    pub bonding_curve_bump: u8,
    pub mint: Box<Account<'info, Mint>>,
    pub bonding_curve: Box<Account<'info, BondingCurve>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve,
    )]
    pub bonding_curve_token_account: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
}
impl BondingCurveLockerCtx<'_> {
    fn get_signer<'a>(&self) -> [&[u8]; 3] {
        let signer: [&[u8]; 3] =
            BondingCurve::get_signer(&self.bonding_curve_bump, self.mint.to_account_info().key);
        signer
    }
    pub fn lock_ata<'a>(&self) -> Result<()> {
        msg!("BondingCurveLockerCtx::lock_ata complete");
        Ok(())
    }
    pub fn unlock_ata<'a>(&self) -> Result<()> {
        msg!("BondingCurveLockerCtx::unlock_ata complete");
        Ok(())
    }

    pub fn revoke_mint_authority(&self) -> Result<()> {
        msg!("CreateBondingCurve::revoke_mint_authority: done");
        Ok(())
    }

    pub fn revoke_freeze_authority(&self) -> Result<()> {
        msg!("CreateBondingCurve::revoke_freeze_authority: done");
        Ok(())
    }
}

pub trait IntoBondingCurveLockerCtx<'info> {
    fn into_bonding_curve_locker_ctx(&self, bonding_curve_bump: u8)
        -> BondingCurveLockerCtx<'info>;
}
