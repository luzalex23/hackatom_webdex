const anchor = require("@coral-xyz/anchor");
const { SystemProgram } = anchor.web3;
const {
  createMint,
  getOrCreateAssociatedTokenAccount,
  getAccount,
  TOKEN_PROGRAM_ID,
} = require("@solana/spl-token");
const assert = require("assert");

describe("hackatom_webdex", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.hackatomWebdex;

  let payer;
  let userAccount;
  let pda;
  let bot;
  let token;

  before(async () => {
    payer = anchor.web3.Keypair.generate();
    userAccount = anchor.web3.Keypair.generate();
    bot = anchor.web3.Keypair.generate().publicKey;
    token = anchor.web3.Keypair.generate().publicKey;

    const airdropSig = await provider.connection.requestAirdrop(
      payer.publicKey,
      1e9 // 1 SOL
    );
    await provider.connection.confirmTransaction(airdropSig);
  });

  it("Deve inicializar o programa", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Transação de inicialização:", tx);
  });

  it("Deve registrar um usuário com sucesso", async () => {
    const tx = await program.methods
      .registerUser()
      .accounts({
        userAccount: userAccount.publicKey,
        user: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([payer, userAccount])
      .rpc();

    const registeredUser = await program.account.user.fetch(userAccount.publicKey);
    assert.strictEqual(registeredUser.gasBalance.toNumber(), 0);
    assert.strictEqual(registeredUser.passBalance.toNumber(), 0);
  });

  it("Deve adicionar GAS com sucesso", async () => {
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

  it("Deve remover GAS com sucesso", async () => {
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

  it("Deve adicionar PASS com sucesso", async () => {
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

  it("Deve executar o rebalance com sucesso", async () => {
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

  it("Deve registrar uma subconta com sucesso", async () => {
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

    const subAcc = await program.account.subAccount.fetch(pda);
    assert.strictEqual(subAcc.owner.toBase58(), payer.publicKey.toBase58());
  });

  it("Deve depositar na subconta com sucesso", async () => {
    await program.methods
      .depositToSubaccount(new anchor.BN(1000))
      .accounts({
        subAccount: pda,
        owner: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const subAcc = await program.account.subAccount.fetch(pda);
    assert.strictEqual(subAcc.balance.toNumber(), 1000);
  });

  it("Deve sacar da subconta com sucesso", async () => {
    await program.methods
      .withdrawFromSubaccount(new anchor.BN(500))
      .accounts({
        subAccount: pda,
        owner: payer.publicKey,
      })
      .signers([payer])
      .rpc();

    const subAcc = await program.account.subAccount.fetch(pda);
    assert.strictEqual(subAcc.balance.toNumber(), 500);
  });

  it("Deve processar um pagamento com sucesso", async () => {
    const token2 = anchor.web3.Keypair.generate().publicKey;
     const bot2 = anchor.web3.Keypair.generate().publicKey;
     const [toPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subaccount"), payer.publicKey.toBuffer(), bot2.toBuffer(), token2.toBuffer()],
      program.programId
     );
     // Registre a subconta de destino
     await program.methods
      .registerSubaccount(bot2, token2)
      .accounts({
      subAccount: toPda,
      owner: payer.publicKey,
      systemProgram: SystemProgram.programId,
      })
      .signers([payer])
      .rpc();
     // Verificar se a conta de destino foi criada corretamente
     const toAcc = await program.account.subAccount.fetch(toPda);
     assert.ok(toAcc);
     // Realizando a transação de pagamento
     await program.methods
     .processPayment(new anchor.BN(200))
     .accounts({
       from: pda,
       toAccount: toPda,
       owner: payer.publicKey,
     })
     .signers([payer])
     .rpc();
     
     // Verificar o saldo após o pagamento
     const fromAcc = await program.account.subAccount.fetch(pda);
     const toAccUpdated = await program.account.subAccount.fetch(toPda);
     assert.strictEqual(fromAcc.balance.toNumber(), 300); // 500 - 200
     assert.strictEqual(toAccUpdated.balance.toNumber(), 200); // 0 + 200
    });
  
  it("Deve mintar tokens para uma conta", async () => {
    const mint = await createMint(
      provider.connection,
      payer,
      payer.publicKey,
      null,
      6
    );

    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
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

    const updatedAccount = await getAccount(provider.connection, tokenAccount.address);
    assert.strictEqual(Number(updatedAccount.amount), 1_000_000);
  });
});
