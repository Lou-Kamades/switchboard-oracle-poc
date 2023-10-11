import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import dotenv from "dotenv";
import { IDL, OraclePoc } from "../target/types/oracle_poc";
dotenv.config();

async function main() {
  console.log(`Adding Oracle`);

  const ORACLE_NAME = "New13";

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = (provider.wallet as anchor.Wallet).payer;
  console.log(`PAYER: ${payer.publicKey}`);

  // TODO: why is anchor workspace empty?
  const program: anchor.Program<OraclePoc> = new anchor.Program(
    IDL,
    new PublicKey("7zNxbvdozQr5zmg6fX3ZpZhWGtoCpUvpSxHXvC25gSWS"),
    provider
  );
  const [programStatePubkey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ORACLEPOC")],
    program.programId
  );
  console.log(`PROGRAM_STATE: ${programStatePubkey}`);

  const oracleBuffer = Buffer.alloc(16);
  oracleBuffer.fill(ORACLE_NAME, 0, Buffer.from(ORACLE_NAME).length);
  console.log(oracleBuffer);
  const [oracle, bump] = PublicKey.findProgramAddressSync(
    [oracleBuffer],
    program.programId
  );

  console.log(`ORACLE: ${oracle}`);

  const signature = await program.methods
    .addOracle({ name: ORACLE_NAME })
    .accounts({
      oracle,
      program: programStatePubkey,
      authority: payer.publicKey,
    })
    .rpc();

  console.log(`Add Oracle: ${signature}`);
}

main();
