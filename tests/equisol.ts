import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Equisol } from "../target/types/equisol";
import { assert } from "chai";

describe("equisol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Equisol as Program<Equisol>;
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet;

  let registryAccount: anchor.web3.Keypair;
  let companyAccount: anchor.web3.Keypair;

  beforeEach(async () => {
    registryAccount = anchor.web3.Keypair.generate();
    companyAccount = anchor.web3.Keypair.generate();
  });

  it("Initializes the program", async () => {
    // Execute the initialize_program function
    await program.methods
      .initializeProgram()
      .accounts({
        registry: registryAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([registryAccount])
      .rpc();

    // Fetch the account data after initialization
    const registry = await program.account.companyRegistry.fetch(
      registryAccount.publicKey
    );

    // Assert that the companies array is initialized empty
    assert.isArray(registry.companies);
    assert.isEmpty(registry.companies);
  });

  it("Initializes the registry", async () => {
    // Call the initialize_registry function
    await program.methods
      .initializeRegistry()
      .accounts({
        registry: registryAccount.publicKey,
        user: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([registryAccount])
      .rpc();

    // Fetch the account and check its state
    const registry = await program.account.companyRegistry.fetch(
      registryAccount.publicKey
    );

    // Assert the companies vector is empty
    assert.isArray(registry.companies);
    assert.lengthOf(registry.companies, 0);
  });

  it("Registers a company", async () => {
    // Initialize the registry first
    await program.methods
      .initializeRegistry()
      .accounts({
        registry: registryAccount.publicKey,
        user: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([registryAccount])
      .rpc();

    // Register a new company
    await program.methods
      .registerCompany("Test Company", new anchor.BN(1000000), new anchor.BN(10), "TST")
      .accounts({
        company: companyAccount.publicKey,
        registry: registryAccount.publicKey,
        user: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([companyAccount])
      .rpc();

    // Fetch the company account and check its state
    const company = await program.account.company.fetch(companyAccount.publicKey);

    assert.equal(company.name, "Test Company");
    assert.equal(company.initialSupply.toString(), "1000000");
    assert.equal(company.initialPrice.toString(), "10");
    assert.equal(company.symbol, "TST");
    assert.equal(company.owner.toString(), wallet.publicKey.toString());

    // Fetch the registry account and check if the company was added
    const registry = await program.account.companyRegistry.fetch(
      registryAccount.publicKey
    );

    assert.isArray(registry.companies);
    assert.lengthOf(registry.companies, 1);
    assert.equal(registry.companies[0].toString(), companyAccount.publicKey.toString());
  });
});
