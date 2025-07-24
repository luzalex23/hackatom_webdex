const assert = require("assert");
const { anchor, program, payer } = require("./setup");

describe("Payment Module", () => {
  const botA = anchor.web3.Keypair.generate().publicKey;
  const botB = anchor.web3.Keypair.generate().publicKey;
  const tokenA = anchor.web3.Keypair.generate().publicKey;
  const tokenB = anchor.web3.Keypair.generate().publicKey;

  let fromPda, toPda;

  it("Prepara subcontas para pagamento", async () => {
    [fromPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subaccount"), payer.publicKey.toBuffer(), botA.toBuffer(), tokenA.toBuffer()],
      program.programId
    );
    [toPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subaccount"), payer.publicKey.toBuffer(), botB.toBuffer(), tokenB.toBuffer()],
      program.programId
    );

    await program.methods.registerSubaccount(botA, tokenA)
      .accounts({ subAccount: fromPda, owner: payer.publicKey, systemProgram: anchor.web3.SystemProgram.programId })
      .signers([payer])
      .rpc();

    await program.methods.registerSubaccount(botB, tokenB)
      .accounts({ subAccount: toPda, owner: payer.publicKey, systemProgram: anchor.web3.SystemProgram.programId })
      .signers([payer])
      .rpc();

    await program.methods.depositToSubaccount(new anchor.BN(300))
      .accounts({ subAccount: fromPda, owner: payer.publicKey })
      .signers([payer])
      .rpc();
  });

  it("Executa um pagamento entre subcontas", async () => {
    await program.methods
      .processPayment(new anchor.BN(200))
      .accounts({
        from: fromPda,
        toAccount: toPda,
        owner: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const fromAcc = await program.account.subAccount.fetch(fromPda);
    const toAcc = await program.account.subAccount.fetch(toPda);
    assert.strictEqual(fromAcc.balance.toNumber(), 100);
    assert.strictEqual(toAcc.balance.toNumber(), 200);
  });

  it("Valida um token permitido", async () => {
    const solToken = "So11111111111111111111111111111111111111112";
    await program.methods
      .validateToken(new anchor.web3.PublicKey(solToken))
      .accounts({})
      .rpc();
  });

it("Realiza saque com taxa", async () => {
  // Buscar saldo antes do saque para depuração
  const toAccBefore = await program.account.subAccount.fetch(toPda);
  console.log("Saldo antes do saque:", toAccBefore.balance.toNumber());

  // Executa saque de 50 com taxa de 10%
  await program.methods
    .withdraw(new anchor.BN(50), new anchor.BN(10))
    .accounts({
      from: toPda,
      owner: payer.publicKey,
    })
    .signers([payer])
    .rpc();

  const toAccAfter = await program.account.subAccount.fetch(toPda);
  assert.strictEqual(toAccAfter.balance.toNumber(), toAccBefore.balance.toNumber() - 50);
});

});
