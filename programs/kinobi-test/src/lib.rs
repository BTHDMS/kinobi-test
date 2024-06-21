use anchor_lang::prelude::*;

declare_id!("3eSEGBd6d6C6imYBaeu9vDJqf4cCjxPE1JkX5TaagJCD");

#[program]
pub mod kinobi_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
       let pda = &mut ctx.accounts.pda;
       pda.data = 0;
       pda.authority = *ctx.accounts.payer.key;
       
       Ok(())
 }

    pub fn set_data(ctx: Context<SetData>, data: u32) -> Result<()> {
        ctx.accounts.pda.data = data;
        Ok(())
 }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
 #[account(mut)]
    payer: Signer<'info>,

 #[account(
 init,
 payer = payer,
 space = 45,
 seeds = [b"example".as_ref(), payer.key().as_ref()],
        bump
 )]
    pda: Account<'info, ExampleStruct>,

    system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct SetData<'info> {
 #[account(mut)]
    authority: Signer<'info>,

 #[account(
        mut,
 has_one = authority,
 seeds = [b"example".as_ref(), authority.key().as_ref()],
        bump
 )]
    pda: Account<'info, ExampleStruct>,
}

#[account]
pub struct ExampleStruct {
    pub data: u32,
    pub authority: Pubkey,
}