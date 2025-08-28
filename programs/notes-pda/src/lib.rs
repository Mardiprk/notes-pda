use anchor_lang::prelude::*;

declare_id!("AgDnpmmCPEMX3RYuXLip8rWdD1eirHjkQ6cCmGRz6j9e");

#[program]
pub mod notes_program {
    use super::*;

    pub fn initialize_note(_ctx: Context<InitializeNote>, content: String) -> Result<()> {
        let note_account = &mut _ctx.accounts.note_account;
        note_account.owner = _ctx.accounts.user.key();
        note_account.content = content;
        msg!("Message Stored on-chain: {}", content);
        Ok(())
    }

    pub fn update_note(_ctx: Context<UpdateNote>, new_content: String) -> Result<()> {
        let note_account = &mut _ctx.accounts.note_account;
        require!(
            note_account.owner == _ctx.accounts.user.key(),
            CustomError::Unauthorized
        );
        require!(new_content.len() < 280, CustomError::TooLong);
        note_account.content = new_content;
        msg!("Message Updated on-chain: {}", new_content);
        Ok(())
    }

    pub fn delete_note(_ctx: Context<DeleteNote>) -> Result<()> {
        let note_account = &mut _ctx.accounts.note_account;
        require!(
            note_account.owner == _ctx.accounts.user.key(),
            CustomError::Unauthorized
        );
        note_account.content.clear();
        msg!("Message Cleared");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(content: String)]
pub struct InitializeNote<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 280,
        seeds = [b"note".key(),user.key().as_ref()],
        bump
    )]
    pub note_account: Account<'info, NoteAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateNote<'info> {
    #[account(
        mut,
        seeds = [b"note".key(),user.key().as_ref()],
        bump
    )]
    pub note_account: Account<'info, NoteAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteNote<'info> {
    #[account(
        mut,
        seeds = [b"note".key(),user.key().as_ref()],
        bump
    )]
    pub note_account: Account<'info, NoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct NoteAccount {
    pub owner: Pubkey,
    pub content: String,
}

#[error_code]
pub enum CustomError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Message cannot be longer than 280 Characters")]
    TooLong,
}
