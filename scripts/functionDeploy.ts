import { Transaction } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { SwitchboardProgram } from "@switchboard-xyz/solana.js";
import dotenv from "dotenv";
import { loadDefaultQueue } from "./utils";
dotenv.config();

async function main() {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = (provider.wallet as anchor.Wallet).payer;
  console.log(`PAYER: ${payer.publicKey}`);

  const switchboardProgram = await SwitchboardProgram.fromProvider(provider);

  const attestationQueueAccount = await loadDefaultQueue(switchboardProgram);
  console.log(`ATTESTATION_QUEUE: ${attestationQueueAccount.publicKey}`);

   // Create the instructions to initialize our Switchboard Function
  const [functionAccount, functionInit] =
  await attestationQueueAccount.createFunctionInstruction(payer.publicKey, {
    name: "pong",
    schedule: "15 * * * * *",
    container: "loukamades/pong",
    containerRegistry: "dockerhub",
    version: "latest"
  });
  console.log(`SWITCHBOARD_FUNCTION: ${functionAccount.publicKey}`);

  const txn = new Transaction()
  txn.add(...functionInit.ixns)
  const signature = await provider.connection.sendTransaction(txn, [payer])

  console.log(`switchboard function deploy: ${signature}`);

}

main()