import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import dotenv from "dotenv";
import { IDL, OraclePoc } from "../target/types/oracle_poc";
dotenv.config();


const ORACLE_NAME = "1";
const ORACLE_MINT = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"); // devnet USDC

async function main() {
  console.log(`Adding Oracle`);

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = (provider.wallet as anchor.Wallet).payer;
  console.log(`PAYER: ${payer.publicKey}`);

  // TODO: why is anchor workspace empty?
  const program: anchor.Program<OraclePoc> = new anchor.Program(
    IDL,
    new PublicKey("GknYjbiQABncTa8JwStdHRX1t1UZArjdAoaRTrccfhdR"),
    provider
  );
  const [programStatePubkey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ORACLEPOC")],
    program.programId
  );
  console.log(`PROGRAM_STATE: ${programStatePubkey}`);

  const [oracleContainer] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("ORACLE")],
    program.programId
  );
  console.log(`ORACLE_CONTAINER: ${oracleContainer}`);

  const signature = await program.methods
    .addOracle({ name: ORACLE_NAME })
    .accounts({
      oracleContainer,
      oracleMint: ORACLE_MINT,
      program: programStatePubkey,
      authority: payer.publicKey,
    })
    .rpc();

  console.log(`Add Oracle: ${signature}`);
}

main();
