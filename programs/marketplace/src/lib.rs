use anchor_lang::prelude::*;

//declare_id!("DpzyKsRTJFJuezVcY7X4BLp4TRiDh3hBtt4CVkAvAR9v");
declare_id!("DSQ41fbL2AVnvBFcH1yvH86AzTLbj2i45RHpwcZbyN9s");
pub mod context;
use context::*;
pub mod state;
use state::*;
pub mod errors;
use errors::*;

#[program]
    pub mod anchor_marketplace {
    use super::*;
    pub fn initialize(mut ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps)?;
        Ok(())
    }
    pub fn list(ctx: Context<List>, price: u64 ) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps);
        ctx.accounts.deposit_nft()?;
        Ok(())
    }
    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        Ok(())
    }
    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol();
        ctx.accounts.send_nft();
        ctx.accounts.close_mint_vault();
        Ok(())
    }
}

//Program path: /home/aymanf10/Turbin3_Q1_25/marketplace/target/deploy/anchor_marketplace.so...
//Program Id: DSQ41fbL2AVnvBFcH1yvH86AzTLbj2i45RHpwcZbyN9s                                                                
//Signature: nTb3RiFmUMNo4HSbm6tyFB4Fp7bHFYt6ZUZrCvFRjQ8xk7421J6kftosQGTkZ7BikE2pshL2gj3DyE8eBnBLngh

