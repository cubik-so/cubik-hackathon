import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CubikHackathon } from "../target/types/cubik_hackathon";

describe("cubik-hackathon", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CubikHackathon as Program<CubikHackathon>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
