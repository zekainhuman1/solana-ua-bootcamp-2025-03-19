import { Keypair } from "@solana/web3.js";

let pref =  "pok".toLowerCase();
let keypair: Keypair
let publicKey: string
do {
 keypair = Keypair.generate();
 publicKey = keypair.publicKey.toBase58().toLowerCase();
} while (!publicKey.startsWith(pref))

console.log(`The public key is: `, keypair.publicKey.toBase58());
console.log(`The secret key is: `, keypair.secretKey);
console.log(`✅ Finished!`);