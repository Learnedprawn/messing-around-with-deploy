use anchor_lang::prelude::*;

declare_id!("ARmbeJQRJ9YuCy2DksDhzzjpy7tmCps22JvtmxNtjwUv");

#[program]
pub mod deploy_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Meow from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
