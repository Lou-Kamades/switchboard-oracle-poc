import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import dotenv from "dotenv";
import { IDL, OraclePoc } from "../target/types/oracle_poc";
import { loadDefaultQueue } from "./utils";
import { SwitchboardProgram } from "@switchboard-xyz/solana.js";
dotenv.config();

async function main() {
  console.log(`Initializing Oracle Proof of Concept`);

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = (provider.wallet as anchor.Wallet).payer;
  console.log(`PAYER: ${payer.publicKey}`);

  const switchboardProgram = await SwitchboardProgram.fromProvider(provider);

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

  console.log(`oracleContainer: ${oracleContainer}`);

  try {
    const programState = await program.account.programState.fetch(
      programStatePubkey
    );
    console.log(`Program state already initialized`);
    console.log(
      `PROGRAM_STATE: \n${JSON.stringify(programState, undefined, 2)}`
    );
    return;

    // Account already initialized
  } catch (error) {
    if (!`${error}`.includes("Account does not exist or has no data")) {
      throw error;
    }
  }

  const attestationQueueAccount = await loadDefaultQueue(switchboardProgram);
  console.log(`ATTESTATION_QUEUE: ${attestationQueueAccount.publicKey}`);

  // Create the instructions to initialize our Switchboard Function
  const [functionAccount, functionInit] =
    await attestationQueueAccount.createFunctionInstruction(payer.publicKey, {
      name: `${process.env.DOCKERHUB_CONTAINER_NAME}`,
      schedule: "15 * * * * *", // TODO: set a real schedule
      container: `${process.env.DOCKERHUB_ORGANIZATION}/${process.env.DOCKERHUB_CONTAINER_NAME}`,
      containerRegistry: "dockerhub",
      version: "latest",
    });
  console.log(`SWITCHBOARD_FUNCTION: ${functionAccount.publicKey}`);

  const signature = await program.methods
    .initialize()
    .accounts({
      program: programStatePubkey,
      oracleContainer,
      authority: payer.publicKey,
      switchboardFunction: functionAccount.publicKey,
    })
    .signers([...functionInit.signers])
    .preInstructions([...functionInit.ixns])
    .rpc();

  console.log(`Oracle Proof of Concept initialized: ${signature}`);
}

main();
