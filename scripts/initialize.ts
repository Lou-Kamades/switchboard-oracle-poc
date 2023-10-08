import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import dotenv from "dotenv";
import { IDL, OraclePoc } from "../target/types/oracle_poc";
dotenv.config();

async function main() {
  console.log(`Initializing Oracle Proof of Concept`);

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = (provider.wallet as anchor.Wallet).payer;
  console.log(`PAYER: ${payer.publicKey}`);

  // TODO: why is anchor workspace empty?
  const program: anchor.Program<OraclePoc> = new anchor.Program(IDL, new PublicKey('A2h16ZekNmvuFzJCS4MdU1Pe1AwZE2pyFtFDBeaRJQES'), provider)
  const [oracle, bump] = PublicKey.findProgramAddressSync([Buffer.from("oracle")], program.programId)

  const signature = await  program.methods.initialize().accounts({
    oracle,
    payer: payer.publicKey,
  }).rpc()

  console.log(`Oracle Proof of Concept initialized: ${signature}`);

}

main()