const anchor = require("@coral-xyz/anchor");
const { SystemProgram } = anchor.web3;
const assert = require("assert");

describe("hackatom_webdex", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  it("Is initialized!", async () => {
    // Add your test here.
    const program = anchor.workspace.hackatomWebdex;
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
describe("Registro de Usuário", () => {
  // Configura o provider com o cluster local (devnet ou localnet)
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.hackatomWebdex;

  it("Deve registrar um usuário com sucesso", async () => {
    const payer = anchor.web3.Keypair.generate(); // Payer do airdrop e assinatura
    const userAccount = anchor.web3.Keypair.generate(); // Conta a ser criada no registro
        
    // Airdrop se necessário (apenas para testes em redes locais/devnet)
    const airdropSig = await provider.connection.requestAirdrop(
      payer.publicKey,
      1e9  // 1 SOL, por exemplo
    );
    await provider.connection.confirmTransaction(airdropSig);    
    // Chama a instrução de registro de usuário
    const tx = await program.rpc.registerUser({
      accounts: {
        userAccount: userAccount.publicKey,
        user: payer.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [payer, userAccount],
    });
    console.log("Transação de registro:", tx);
    
    
    //ler a conta e verificar se os dados foram persistidos corretamente.
    const registeredUser = await program.account.user.fetch(userAccount.publicKey);
    assert.ok(registeredUser.gasBalance.toNumber() === 0);
    assert.ok(registeredUser.passBalance.toNumber() === 0);
    
  });
});