import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tokenomics } from "../target/types/tokenomics";

describe("tokenomics", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Tokenomics as Program<Tokenomics>;
  const connection = program.provider.connection;

  it("Is initialized!", async () => {
    // Generate a new keypair for the tokenomics account using PDA
    const [tokenomicsAccount] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("tokenomics_account")],
      program.programId
    );

    // Define the fee account (mocked for testing)
    const [feeAccountPda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("fee_account"), tokenomicsAccount.toBuffer()], // Use tokenomicsAccount as seed
      program.programId
    );

    // Define the provider's wallet as the authority
    const authority = program.provider.publicKey;

    // Check if the tokenomicsAccount already exists
    const tokenomicsAccountInfo = await connection.getAccountInfo(tokenomicsAccount);

    if (tokenomicsAccountInfo) {
      console.log("Tokenomics account already exists. Skipping initialization.");
      return;
    } else {
      console.log("Tokenomics account does not exist. Proceeding with initialization.");
    }

    // Invoke the initialize method
    const tx = await program.methods
      .initialize(new anchor.BN(1000)) // Replace with the desired fee_rate
      .accounts({
        tokenomicsAccount: tokenomicsAccount, // Use the derived PDA
        authority: authority,
        feeAccount: feeAccountPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
