const assert = require("assert");
const { anchor, program, payer } = require("./setup");
const {
  createMint,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  TOKEN_PROGRAM_ID,
} = require("@solana/spl-token");

describe("Faucet Module", () => {
  it("Mint de tokens SPL", async () => {
    const mint = await createMint(
      program.provider.connection,
      payer,
      payer.publicKey,
      null,
      6
    );

    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      program.provider.connection,
      payer,
      mint,
      payer.publicKey
    );

    await program.methods
      .mintToken(new anchor.BN(1_000_000))
      .accounts({
        mint: mint,
        recipient: tokenAccount.address,
        authority: payer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([payer])
      .rpc();

    const updatedAccount = await getAccount(
      program.provider.connection,
      tokenAccount.address
    );
    assert.strictEqual(Number(updatedAccount.amount), 1_000_000);
  });
});
