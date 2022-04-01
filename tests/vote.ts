import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert, expect, use } from "chai";
import { SimpleVote } from "../target/types/simple_vote";

describe("simple_vote", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SimpleVote as Program<SimpleVote>;
  const candidateKeyPair = anchor.web3.Keypair.generate();
  const user = program.provider.wallet;
  const other = anchor.web3.Keypair.generate();

  it("setup vote platform!", async () => {
    await program.rpc.setupVotePlatform({
      accounts: {
        candidate: candidateKeyPair.publicKey,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [candidateKeyPair],
    });

    let candidate = await program.account.candidate.fetch(
      candidateKeyPair.publicKey
    );
    expect(candidate.peopleWhoVoted.length).to.equal(0);
    expect(candidate.authority.toString()).to.equal(user.publicKey.toString());
    
});

  it("register candidate", async () => {
    await program.rpc.registerCandidate("Talyer Swift", 35, "CEO", {
      accounts: {
        candidate: candidateKeyPair.publicKey,
        authority: user.publicKey,
      }
    });
    let candidate = await program.account.candidate.fetch(
      candidateKeyPair.publicKey
    );
    expect(candidate.name).to.equal("Talyer Swift");
    expect(candidate.age).to.equal(35);
    expect(candidate.des.toString()).to.equal("CEO");
  });

  it("can't change candidate info by others", async () => {
   try {
    await program.rpc.registerCandidate("Tal Swift", 35, "CEO", {
        accounts: {
          candidate: candidateKeyPair.publicKey,
          authority: other.publicKey,
        },
      });
      assert.ok(false)
   } catch (error) {
       expect(error.toString()).to.equal("Error: Signature verification failed")
   }
  });
});