import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import dotenv from "dotenv";
import { IDL, OraclePoc } from "../target/types/oracle_poc";
dotenv.config();

async function main() {
  console.log(`Adding Oracle`);

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = (provider.wallet as anchor.Wallet).payer;
  console.log(`PAYER: ${payer.publicKey}`);

  const program: anchor.Program<OraclePoc> = new anchor.Program(
    IDL,
    new PublicKey("GknYjbiQABncTa8JwStdHRX1t1UZArjdAoaRTrccfhdR"),
    provider
  );
  const goo = await program.account.oracleContainer.fetch(
    new PublicKey("3mg6R9XkNnggVhmmUgNVNCbhfhJVA1QNuFuMYuixDPgk")
  );

  const x = await provider.connection.getAccountInfo( new PublicKey("3mg6R9XkNnggVhmmUgNVNCbhfhJVA1QNuFuMYuixDPgk"))
  console.log(x);

  

  // const ORACLE_NAME = "test";
  // const oracleBuffer = Buffer.alloc(16);
  // oracleBuffer.fill(ORACLE_NAME, 0, Buffer.from(ORACLE_NAME).length);
  // console.log(oracleBuffer);
}

main();
