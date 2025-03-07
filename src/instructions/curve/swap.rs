use std::ops::Div;

use anchor_lang::{prelude::*, solana_program::system_instruction};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    errors::ContractError,
    events::*,
    state::{bonding_curve::*, fee_vault::FeeVault, global::*},
};

use crate::state::bonding_curve::locker::{BondingCurveLockerCtx, IntoBondingCurveLockerCtx};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SwapParams {
    pub base_in: bool,
    pub exact_in_amount: u64,
    pub min_out_amount: u64,
}

#[event_cpi]
#[derive(Accounts)]
#[instruction(params: SwapParams)]
pub struct Swap<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        seeds = [Global::SEED_PREFIX.as_bytes()],
        constraint = global.initialized == true @ ContractError::NotInitialized,
        bump,
    )]
    global: Box<Account<'info, Global>>,

    mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [BondingCurve::SEED_PREFIX.as_bytes(), mint.to_account_info().key.as_ref()],
        constraint = bonding_curve.complete == false @ ContractError::BondingCurveComplete,
        bump,
    )]
    bonding_curve: Box<Account<'info, BondingCurve>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = bonding_curve,
    )]
    bonding_curve_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [FeeVault::SEED_PREFIX.as_bytes(), mint.to_account_info().key.as_ref()],
        bump,
    )]
    fee_vault: Box<Account<'info, FeeVault>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    user_token_account: Box<Account<'info, TokenAccount>>,

    system_program: Program<'info, System>,

    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,

    clock: Sysvar<'info, Clock>,
}
impl<'info> IntoBondingCurveLockerCtx<'info> for Swap<'info> {
    fn into_bonding_curve_locker_ctx(
        &self,
        bonding_curve_bump: u8,
    ) -> BondingCurveLockerCtx<'info> {
        BondingCurveLockerCtx {
            bonding_curve_bump,
            mint: self.mint.clone(),
            bonding_curve: self.bonding_curve.clone(),
            bonding_curve_token_account: self.bonding_curve_token_account.clone(),
            token_program: self.token_program.clone(),
        }
    }
}
impl Swap<'_> {
    pub fn validate(&self, params: &SwapParams) -> Result<()> {
        let SwapParams {
            base_in: _,
            exact_in_amount,
            min_out_amount: _,
        } = params;
        let clock = Clock::get()?;

        require!(
            self.bonding_curve.is_started(&clock),
            ContractError::CurveNotStarted
        );
        require!(exact_in_amount > &0, ContractError::MinSwap);
        Ok(())
    }
    pub fn handler(ctx: Context<Swap>, params: SwapParams) -> Result<()> {
        // Contact me in TG : asseph_1994 
        BondingCurve::invariant(
            &mut ctx
                .accounts
                .into_bonding_curve_locker_ctx(ctx.bumps.bonding_curve),
        )?;
        let bonding_curve = &ctx.accounts.bonding_curve;
        emit_cpi!(TradeEvent {
          
        });
        if bonding_curve.complete {
            emit_cpi!(CompleteEvent {
              
            });
        }

        msg!("{:#?}", bonding_curve);

        Ok(())
    }

    pub fn complete_buy(
        ctx: &Context<Swap>,
        buy_result: BuyResult,
        min_out_amount: u64,
        fee_lamports: u64,
    ) -> Result<()> {
        msg!("fee_lamports: {}", fee_lamports);
        // Contact me in TG : asseph_1994 

        Ok(())
    }

    pub fn complete_sell(
        ctx: &Context<Swap>,
        sell_result: SellResult,
        min_out_amount: u64,
        fee_lamports: u64,
    ) -> Result<()> {
        
        // Contact me in TG : asseph_1994 
        
        ctx.accounts
            .bonding_curve
            .sub_lamports(fee_lamports)
            .unwrap();
        ctx.accounts.fee_vault.add_lamports(fee_lamports).unwrap();
        msg!("Fee to fee_vault transfer complete");
        Ok(())
    }
}
