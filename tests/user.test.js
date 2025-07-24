const assert = require("assert");
const { anchor, program, payer, userAccount } = require("./setup");
const { SystemProgram } = anchor.web3;

describe("User Module", () => {
  it("Registra usuário", async () => {
    await program.methods
      .registerUser()
      .accounts({
        userAccount: userAccount.publicKey,
        user: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([payer, userAccount])
      .rpc();

    const user = await program.account.user.fetch(userAccount.publicKey);
    assert.strictEqual(user.gasBalance.toNumber(), 0);
    assert.strictEqual(user.passBalance.toNumber(), 0);
  });

  it("Adiciona GAS", async () => {
    await program.methods
      .addGas(new anchor.BN(500))
      .accounts({
        userAccount: userAccount.publicKey,
        owner: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const user = await program.account.user.fetch(userAccount.publicKey);
    assert.strictEqual(user.gasBalance.toNumber(), 500);
  });

  it("Remove GAS", async () => {
    await program.methods
      .removeGas(new anchor.BN(200))
      .accounts({
        userAccount: userAccount.publicKey,
        owner: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const user = await program.account.user.fetch(userAccount.publicKey);
    assert.strictEqual(user.gasBalance.toNumber(), 300);
  });

  it("Adiciona PASS", async () => {
    await program.methods
      .addPass(new anchor.BN(100))
      .accounts({
        userAccount: userAccount.publicKey,
        owner: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const user = await program.account.user.fetch(userAccount.publicKey);
    assert.strictEqual(user.passBalance.toNumber(), 100);
  });

  it("Executa rebalance", async () => {
    await program.methods
      .rebalance()
      .accounts({
        userAccount: userAccount.publicKey,
        owner: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const user = await program.account.user.fetch(userAccount.publicKey);
    assert.ok(user.gasBalance.toNumber() >= 0);
    assert.ok(user.passBalance.toNumber() >= 0);
  });
});
