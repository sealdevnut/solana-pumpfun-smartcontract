use crate::errors::ContractError;
// use crate::state::allocation::AllocationData;
use crate::state::bonding_curve::locker::BondingCurveLockerCtx;
use crate::state::bonding_curve::*;
use crate::util::{bps_mul, bps_mul_raw};
use crate::Global;
use anchor_lang::prelude::*;
use std::fmt::{self};
use structs::BondingCurve;

impl BondingCurve {
    pub const SEED_PREFIX: &'static str = "bonding-curve";

    pub fn get_signer<'a>(bump: &'a u8, mint: &'a Pubkey) -> [&'a [u8]; 3] {
        [
            Self::SEED_PREFIX.as_bytes(),
            mint.as_ref(),
            std::slice::from_ref(bump),
        ]
    }

    pub fn update_from_params(
        &mut self,
        mint: Pubkey,
        creator: Pubkey,
        global_config: &Global,
        params: &CreateBondingCurveParams,
        clock: &Clock,
        bump: u8,
    ) -> &mut Self {
        Ok(())
    }

    pub fn get_buy_price(&self, tokens: u64) -> Option<u64> {
        // Contact me in TG : asseph_1994 
        Ok(())
    }

    pub fn apply_buy(&mut self, sol_amount: u64) -> Option<BuyResult> {
        Ok(())
    }

    pub fn get_sell_price(&self, tokens: u64) -> Option<u64> {
        Ok(())
    }

    pub fn apply_sell(&mut self, token_amount: u64) -> Option<SellResult> {
        Ok(())
    }

    pub fn get_tokens_for_buy_sol(&self, sol_amount: u64) -> Option<u64> {
        Ok(())
    }

    pub fn get_tokens_for_sell_sol(&self, sol_amount: u64) -> Option<u64> {
        Ok(())
    }

    pub fn is_started(&self, clock: &Clock) -> bool {
        Ok(())
    }

    pub fn invariant<'info>(ctx: &mut BondingCurveLockerCtx<'info>) -> Result<()> {
        Ok(())
    }
}

impl fmt::Display for BondingCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BondingCurve {{ creator: {:?}, initial_virtual_token_reserves: {:?}, virtual_sol_reserves: {:?}, virtual_token_reserves: {:?}, real_sol_reserves: {:?}, real_token_reserves: {:?}, token_total_supply: {:?}, start_time: {:?}, complete: {:?} }}",
            self.creator,
            self.initial_virtual_token_reserves,
            self.virtual_sol_reserves,
            self.virtual_token_reserves,
            self.real_sol_reserves,
            self.real_token_reserves,
            self.token_total_supply,
            self.start_time,
            self.complete
        )
    }
}
