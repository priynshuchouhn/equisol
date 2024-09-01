use anchor_lang::prelude::*;

// This macro declares the program ID for the smart contract. 
// It must match the program ID in the deployed Solana program.
declare_id!("EYkmzAxdQ6ivpuPJRQG25nwgp2aA6ZHLfv4iMz3fw72r");

#[program]
pub mod equisol {
    use super::*;

    pub fn initialize_program(ctx: Context<InitializeRegistry>) -> Result<()> {
        // Check if the registry has already been initialized
        let registry = &ctx.accounts.registry;
        if !registry.companies.is_empty() {
            return Err(ErrorCode::AlreadyInitialized.into());
        }

        // If not initialized, proceed with the initialization
        initialize_registry(ctx)
    }

    // This function initializes a new company registry on the blockchain.
    // The registry will store the public keys of all registered companies.
    pub fn initialize_registry(ctx: Context<InitializeRegistry>) -> Result<()> {
        // Get a mutable reference to the registry account.
        let registry = &mut ctx.accounts.registry;

        // Initialize the registry with an empty list of companies.
        registry.companies = Vec::new();

        // Return Ok to indicate successful execution.
        Ok(())
    }

    // This function registers a new company on the blockchain.
    // It requires the company name, initial supply of shares, initial price, and a symbol.
    pub fn register_company(
        ctx: Context<RegisterCompany>,
        name: String,
        initial_supply: u64,
        initial_price: u64,
        symbol: String,
    ) -> Result<()> {
        // Get mutable references to the company and registry accounts.
        let company = &mut ctx.accounts.company;
        let registry = &mut ctx.accounts.registry;

        // Set the company details provided by the user.
        company.name = name;
        company.initial_supply = initial_supply;
        company.initial_price = initial_price;
        company.symbol = symbol;

        // The owner of the company is set to the public key of the user who called this function.
        company.owner = ctx.accounts.user.key();

        // Add the public key of the newly created company to the registry's list of companies.
        registry.companies.push(company.key());

        // Return Ok to indicate successful execution.
        Ok(())
    }
}

// The `Company` struct defines the data structure that represents a company on the blockchain.
// Each company has a name, initial supply and price of shares, a symbol, the owner's public key, and the share mint public key.
#[account]
pub struct Company {
    pub name: String,          // Name of the company
    pub initial_supply: u64,   // Initial supply of shares
    pub initial_price: u64,    // Initial price per share
    pub symbol: String,        // Symbol representing the company's shares
    pub owner: Pubkey,         // Public key of the company's owner
    pub share_mint: Pubkey,    // Public key of the mint for the company's shares
}

// The `CompanyRegistry` struct defines the data structure for the registry that stores company public keys.
#[account]
pub struct CompanyRegistry {
    pub companies: Vec<Pubkey>, // A list of public keys, each representing a registered company
}

// This struct defines the accounts required to initialize the company registry.
#[derive(Accounts)]
pub struct InitializeRegistry<'info> {
    #[account(init, payer = user, space = 8 + 1024)] // Initialize the `registry` account, payer is `user`, allocate space (adjust as needed)
    pub registry: Account<'info, CompanyRegistry>,   // Account to store the registry of companies
    #[account(signer, mut)]                          // `user` must sign the transaction, mutable because they pay for the account creation
    pub user: Signer<'info>,                         // The user who is initializing the registry
    pub system_program: Program<'info, System>,      // Reference to the Solana system program (required for account creation)
}

// This struct defines the accounts required to register a new company.
#[derive(Accounts)]
pub struct RegisterCompany<'info> {
    #[account(init, payer = user, space = 8 + 256)] // Initialize the `company` account, payer is `user`, allocate space (adjust as needed)
    pub company: Account<'info, Company>,           // Account to store company data
    #[account(mut)]                                 // `registry` account is mutable because we're adding the company to it
    pub registry: Account<'info, CompanyRegistry>,  // Account to store the registry of companies
    #[account(signer, mut)]                         // `user` must sign the transaction, mutable because they pay for the account creation
    pub user: Signer<'info>,                        // The user who is registering the company
    pub system_program: Program<'info, System>,     // Reference to the Solana system program (required for account creation)
}

#[error_code]
pub enum ErrorCode {
    #[msg("The registry has already been initialized.")]
    AlreadyInitialized,
}