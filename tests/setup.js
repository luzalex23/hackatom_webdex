const anchor = require("@coral-xyz/anchor");
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.hackatomWebdex;

const fs = require("fs");
const path = require("path");
const walletPath = path.join(process.env.HOME, ".config/solana/id.json");
const secret = JSON.parse(fs.readFileSync(walletPath));
const payer = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(secret));
const userAccount = anchor.web3.Keypair.generate();
const bot = anchor.web3.Keypair.generate().publicKey;
const token = anchor.web3.Keypair.generate().publicKey;

module.exports = {
  anchor,
  provider,
  program,
  payer,
  userAccount,
  bot,
  token,
};
