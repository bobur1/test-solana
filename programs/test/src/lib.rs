use anchor_lang::prelude::*;
use solana_security_txt::security_txt;

declare_id!("7azoMVxmRym7cd6zFoGN82p45cwuqhoS91owLvQGFXsP");

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Test Solana Program",
    project_url: "https://github.com/bobur1/test-solana",
    contacts: "email:derbibme@gmail.com",
    policy: "https://github.com/bobur1/test-solana/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/bobur1/test-solana"
}

#[program]
pub mod test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
