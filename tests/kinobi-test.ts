import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { KinobiTest } from "../target/types/kinobi_test";

describe("kinobi-test", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.KinobiTest as Program<KinobiTest>;
  const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
 [
      Buffer.from("example"),
      program.provider.publicKey.toBuffer()
 ],
    program.programId
 )
  it("Is initialized!", async () => {
    const tx = await program.methods
 .initialize()
 .accountsStrict({
        payer: program.provider.publicKey,
        pda,
        systemProgram: anchor.web3.SystemProgram.programId
 })
 .rpc();
 });
  it("Can set data!", async () => {
    const tx = await program.methods
 .setData(10)
 .accountsStrict({
        authority: program.provider.publicKey,
        pda
 })
 .rpc({skipPreflight: true});
 });
});