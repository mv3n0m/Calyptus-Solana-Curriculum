use anchor_lang::prelude::*;

declare_id!("Cn8qk44ifkVHXRohXJWJuqwd8NvdZDL3r9Eai81i9FcY");

#[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
