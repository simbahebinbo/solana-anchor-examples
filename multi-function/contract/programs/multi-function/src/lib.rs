use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTW3v2RkMNvCPZcD7RMJLVX4mP");

#[program]
mod multi_function {
    use super::*;

    pub fn function_a(ctx: Context<FunctionA>, field1: u64) -> Result<()> {
        msg!("Called function_a with field1: {}", field1);
        Ok(())
    }

    pub fn function_b(ctx: Context<FunctionB>, field2: String) -> Result<()> {
        msg!("Called function_b with field2: {}", field2);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FunctionA<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct FunctionB<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}
