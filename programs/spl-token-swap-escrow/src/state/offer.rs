use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct OfferState {
    pub id: u64,
    pub maker: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub token_b_wanted_amount: u64,
    pub offer_bump: u8,
}
//space=8+32+32+32+8+1