import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tokenomics } from "../target/types/tokenomics";



describe("tokenomics", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Tokenomics as Program<Tokenomics>;

  it("Is initialized!", async () => {
    // Generate a new keypair for the tokenomics account
 
    const [tokenomicsAccount] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("tokenomics_account")],
      program.programId
    );
    

    // Define the fee account (mocked for testing)
    const feeAccount = anchor.web3.Keypair.generate();

    // Define the provider's wallet as the authority
    const authority = program.provider.publicKey;

   

    // Invoke the initialize method
    const tx = await program.methods
    .initialize(new anchor.BN(1000)) // Replace with the desired fee_rate
    .accounts({
      tokenomicsAccount: tokenomicsAccount, // Use the derived PDA
      authority: authority,
      feeAccount: feeAccount.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
  
  console.log("Your transaction signature", tx);
  });
});
