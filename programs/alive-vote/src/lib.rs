use anchor_lang::prelude::*;

declare_id!("BmRad81tyedXignzFL9NEDzQmnDVpcUDvc3Ny8q7gKU1");

#[program]
pub mod alive_vote {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
