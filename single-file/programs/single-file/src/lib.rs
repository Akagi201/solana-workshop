use anchor_lang::prelude::*;

declare_id!("CiGaMBEdHyoDQFp59xXmafxYjGTyKuDXDUY3dP2j6ySd");

#[program]
pub mod single_file {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
