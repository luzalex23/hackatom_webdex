const assert = require("assert");
const { anchor, program, payer, bot, token } = require("./setup");
const { SystemProgram } = anchor.web3;

describe("SubAccount Module", () => {
  let pda;

  it("Registra uma subconta", async () => {
    [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subaccount"), payer.publicKey.toBuffer(), bot.toBuffer(), token.toBuffer()],
      program.programId
    );

    await program.methods
      .registerSubaccount(bot, token)
      .accounts({
        subAccount: pda,
        owner: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([payer])
      .rpc();

    const acc = await program.account.subAccount.fetch(pda);
    assert.strictEqual(acc.owner.toBase58(), payer.publicKey.toBase58());
  });

  it("Deposita na subconta", async () => {
    await program.methods
      .depositToSubaccount(new anchor.BN(1000))
      .accounts({
        subAccount: pda,
        owner: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const acc = await program.account.subAccount.fetch(pda);
    assert.strictEqual(acc.balance.toNumber(), 1000);
  });

  it("Saca da subconta", async () => {
    await program.methods
      .withdrawFromSubaccount(new anchor.BN(500))
      .accounts({
        subAccount: pda,
        owner: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const acc = await program.account.subAccount.fetch(pda);
    assert.strictEqual(acc.balance.toNumber(), 500);
  });

  it("Consulta informações da subconta", async () => {
    const info = await program.account.subAccount.fetch(pda);
    assert.strictEqual(info.bot.toBase58(), bot.toBase58());
    assert.strictEqual(info.token.toBase58(), token.toBase58());
  });
});
