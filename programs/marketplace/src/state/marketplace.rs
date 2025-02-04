use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey, // 32 bytes
    pub fee: u16, // 2 bytes
    pub bump: u8, // 1
    pub treasury_bump: u8, // collect the fee
    pub rewards_bump: u8, // to give out rewards
    pub name: String,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + 1 +1 + 1 + (4 + 32); // The initial 8 bytes are allocated for a unique account discriminator, which is used by Anchor to uniquely identify and validate the account structure
}
