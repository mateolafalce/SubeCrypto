import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SubeCrypto } from "../target/types/sube_crypto";
import { SystemProgram, PublicKey } from "@solana/web3.js";

describe("Register a business", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.SubeCrypto as Program<SubeCrypto>;
  const programId = program.programId;

  it("Is initialized!", async () => {
    const sube = PublicKey.findProgramAddressSync(
      [payer.publicKey.toBuffer()],
      programId
    )[0];

    const tx = await program.methods
      .takeTrip(
        2,
      )
      .accounts({
        sube: sube,
        from: payer.publicKey,
        to: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("Transaction signature", tx);
  });
});
