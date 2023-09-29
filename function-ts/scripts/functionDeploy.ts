import { Connection, Keypair, Transaction } from "@solana/web3.js";
import { parseMrEnclave } from "@switchboard-xyz/common";
import { SwitchboardProgram, SwitchboardWalletFundParams } from "@switchboard-xyz/solana.js";
import {
  AttestationQueueAccount,
  FunctionAccount,
} from "@switchboard-xyz/solana.js";
import * as fs from 'node:fs'

async function main() {

    const RPC_URL=''
    const KEYPAIR_PATH=''
    const connection = new Connection(RPC_URL)

    const secretKey = JSON.parse(fs.readFileSync(KEYPAIR_PATH, 'utf-8'))
    const kp = Keypair.fromSecretKey(Uint8Array.from(secretKey));

    const program = await SwitchboardProgram.load(
        connection,
        kp
        );

    const attestationQueueAccount = new AttestationQueueAccount(
        program,
        "CkvizjVnm2zA5Wuwan34NhVT3zFc7vqUyGnA6tuEF5aE" // devnet attestation queue
    );

    // const [functionAccount] = await FunctionAccount.create(program, {
    //     name: "RUST_ORACLE2",
    //     metadata: "updates the oracle counter",
    //     schedule: "30 * * * * *", // every 30 seconds
    //     container: "loukamades/poc-switchboard-oracle",
    //     containerRegistry: "dockerhub",
    //     version: "latest",
    //     mrEnclave: parseMrEnclave("0x58da7605bd9e55284fb3a2024c5cfff07b405391d5e819b584869e8d959e1e7c"), // run 'make measurement' and pass in here
    //     attestationQueue: attestationQueueAccount,
    // });

    const [functionAccount, functionInit] =
    await attestationQueueAccount.createFunctionInstruction(kp.publicKey, {
      name: "rust_Ix_test",
      schedule: "15 * * * * *",
      container: "loukamades/poc-switchboard-oracle",
      containerRegistry: "dockerhub",
      version: `latest`,
      mrEnclave: parseMrEnclave("0xa814a864bf5aac27751ae2c60fdece42f570424dbde384e171395633502bdfe9")
    });
    console.log(`SWITCHBOARD_FUNCTION: ${functionAccount.publicKey}`);

    const txn = new Transaction()
    txn.add(...functionInit.ixns)
    const signature = await connection.sendTransaction(txn, [kp])

    console.log(signature)

}

main()