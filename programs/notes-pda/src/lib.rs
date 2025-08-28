use anchor_lang::prelude::*;

declare_id!("AgDnpmmCPEMX3RYuXLip8rWdD1eirHjkQ6cCmGRz6j9e");

#[program]
pub mod notes_pda {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
