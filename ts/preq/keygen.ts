// import { Keypair } from "@solana/web3.js";
import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

//Generate a new keypair
let kp = Keypair.generate();
console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}
To save your wallet, copy and paste the following into a JSON file:
[${kp.secretKey}]`);
