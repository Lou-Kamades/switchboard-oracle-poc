import { Connection, Keypair } from "@solana/web3.js";
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

    const secretKey = JSON.parse(fs.readFileSync(KEYPAIR_PATH, 'utf-8'))
    const kp = Keypair.fromSecretKey(Uint8Array.from(secretKey));

    const program = await SwitchboardProgram.load(
        new Connection(RPC_URL),
        kp
        );

    const attestationQueueAccount = new AttestationQueueAccount(
        program,
        "CkvizjVnm2zA5Wuwan34NhVT3zFc7vqUyGnA6tuEF5aE" // devnet attestation queue
    );

    const [functionAccount] = await FunctionAccount.create(program, {
        name: "UPDATE_ORACLE",
        metadata: "updates the oracle counter",
        schedule: "30 * * * * *", // every 30 seconds
        container: "loukamades/poc-switchboard-oracle",
        containerRegistry: "dockerhub",
        version: "latest",
        mrEnclave: parseMrEnclave("0xc975678a533fb8cfe8b790c96d400fdecaf3f2bb580abf32d849909122ccdabc"), // run 'make measurement' and pass in here
        attestationQueue: attestationQueueAccount,
    });

    console.log(functionAccount.publicKey.toBase58())

}

main()