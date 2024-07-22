/// Imports necessary items from the Anchor framework.
use anchor_lang::prelude::*;

// This is your program's public key and it will update automatically when you build the project.
// The `declare_id!` macro sets the program's unique identifier, which is essential for deploying
// and interacting with the program on the Solana blockchain.
declare_id!("4yt2ZeKvCQYGKCnG8WoibHSebf5d5pGZWCeALTHMZZ71");

/// The main program module for the journal.
/// The `#[program]` attribute macro defines the entry points for the Solana program.
#[program]
pub mod journal {
    use super::*;

    /// Creates a new journal entry.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context containing the accounts involved in the transaction.
    /// * `title` - The title of the journal entry.
    /// * `message` - The message of the journal entry.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Returns an empty result on success.
    ///
    /// This function initializes a new journal entry account with the provided title and message.
    /// It logs the creation of the entry and assigns the owner's public key to the entry.
    pub fn create_journal_entry(
        ctx: Context<CreateEntry>,
        title: String,
        message: String,
    ) -> Result<()> {
        // Log messages to the Solana runtime, useful for debugging.
        msg!("Journal Entry Created");
        msg!("Title: {}", title);
        msg!("Message: {}", message);

        // Access the mutable reference to the journal entry account.
        let journal_entry = &mut ctx.accounts.journal_entry;
        // Set the owner of the journal entry to the public key of the transaction signer.
        journal_entry.owner = ctx.accounts.owner.key();
        // Set the title and message of the journal entry.
        journal_entry.title = title;
        journal_entry.message = message;
        Ok(())
    }

    /// Updates an existing journal entry.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context containing the accounts involved in the transaction.
    /// * `title` - The new title of the journal entry.
    /// * `message` - The new message of the journal entry.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Returns an empty result on success.
    ///
    /// This function updates the message of an existing journal entry account with the provided message.
    /// It logs the update of the entry.
    pub fn update_journal_entry(
        ctx: Context<UpdateEntry>,
        title: String,
        message: String,
    ) -> Result<()> {
        // Log messages to the Solana runtime, useful for debugging.
        msg!("Journal Entry Updated");
        msg!("Title: {}", title);
        msg!("Message: {}", message);

        // Access the mutable reference to the journal entry account.
        let journal_entry = &mut ctx.accounts.journal_entry;
        // Update the message of the journal entry.
        journal_entry.message = message;

        Ok(())
    }

    /// Deletes an existing journal entry.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context containing the accounts involved in the transaction.
    /// * `title` - The title of the journal entry to be deleted.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Returns an empty result on success.
    ///
    /// This function deletes an existing journal entry account.
    /// It logs the deletion of the entry.
    #[allow(unused_variables)]
    pub fn delete_journal_entry(ctx: Context<DeleteEntry>, title: String) -> Result<()> {
        // Log the deletion message to the Solana runtime, useful for debugging.
        msg!("Journal entry titled {} deleted", title);

        Ok(())
    }
}

/// Represents the state of a journal entry.
/// The `#[account]` attribute macro defines a struct that will be stored on-chain.
/// The `#[derive(InitSpace)]` attribute macro is used to initialize the account with a space of 8 bytes.
#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {
    /// The public key of the owner of the journal entry.
    pub owner: Pubkey,
    /// The title of the journal entry. Maximum length is 50 characters.
    #[max_len(50)]
    pub title: String,
    /// The message of the journal entry. Maximum length is 1000 characters.
    #[max_len(1000)]
    pub message: String,
}

/// The context for the `create_journal_entry` function.
/// The `#[derive(Accounts)]` attribute macro defines the accounts required for the function.
#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct CreateEntry<'info> {
    /// The account to be created or initialized for the journal entry.
    ///
    /// - `init_if_needed`: Initializes the account if it doesn't already exist.
    /// - `seeds`: A unique identifier for the account, derived from the title and owner's public key.
    /// - `bump`: A nonce used to ensure the uniqueness of the derived address.
    /// - `payer`: The account that will pay for the account creation.
    /// - `space`: The amount of space to allocate for the account.
    #[account(
        init_if_needed,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        payer = owner,
        space = 8 + JournalEntryState::INIT_SPACE
    )]
    pub journal_entry: Account<'info, JournalEntryState>,
    /// The signer of the transaction.
    /// This account must sign the transaction to authorize it.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// The system program required for account creation.
    /// This is a built-in program that provides basic account management functionalities.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, message: String)]
pub struct UpdateEntry<'info> {
    /// The account to be updated for the journal entry.
    ///
    /// - `mut`: The account is mutable, meaning it can be modified.
    /// - `seeds`: A unique identifier for the account, derived from the title and owner's public key.
    /// - `bump`: A nonce used to ensure the uniqueness of the derived address.
    /// - `realloc`: Reallocates the account with the new size.
    /// - `realloc::payer`: The account that will pay for the reallocation.
    /// - `realloc::zero`: Ensures the newly allocated space is zeroed out.
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + 32 + 1 + 4 + title.len() + 4 + message.len(),
        realloc::payer = owner,
        realloc::zero = true,
    )]
    pub journal_entry: Account<'info, JournalEntryState>,
    /// The signer of the transaction.
    /// This account must sign the transaction to authorize it.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// The system program required for account reallocation.
    /// This is a built-in program that provides basic account management functionalities.
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {
    /// The account to be deleted for the journal entry.
    ///
    /// - `mut`: The account is mutable, meaning it can be modified.
    /// - `seeds`: A unique identifier for the account, derived from the title and owner's public key.
    /// - `bump`: A nonce used to ensure the uniqueness of the derived address.
    /// - `close`: Closes the account and transfers the remaining lamports to the specified account.
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        close = owner,
    )]
    pub journal_entry: Account<'info, JournalEntryState>,
    /// The signer of the transaction.
    /// This account must sign the transaction to authorize it.
    #[account(mut)]
    pub owner: Signer<'info>,
    /// The system program required for account closure.
    /// This is a built-in program that provides basic account management functionalities.
    pub system_program: Program<'info, System>,
}
