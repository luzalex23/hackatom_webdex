const assert = require("assert");
const { anchor, program, provider, payer } = require("./setup");
const { SystemProgram } = anchor.web3;
describe("Bot Module", () => {
  const botKey = anchor.web3.Keypair.generate();

 it("Cria um bot com sucesso", async () => {
    const tx = await program.methods
      .createBot("prefix42", "Bot_Alpha", payer.publicKey, payer.publicKey, payer.publicKey, payer.publicKey, payer.publicKey)
      .accounts({
        bot: botKey.publicKey,
        admin: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([payer, botKey])
      .rpc();

    console.log("Transação:", tx);

    const bot = await program.account.bot.fetch(botKey.publicKey);
    assert.strictEqual(bot.name, "Bot_Alpha");
    assert.strictEqual(bot.prefix, "prefix42");
  });

  it("Consulta informações do bot", async () => {
    const info = await program.account.bot.fetch(botKey.publicKey);
    assert.strictEqual(info.owner.toBase58(), payer.publicKey.toBase58());
  });
});
