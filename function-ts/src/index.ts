import { FunctionRunner } from "@switchboard-xyz/solana.js/runner";
import { FatOracle } from "./types";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { UpdateOracleAccounts, updateOracle } from "./sdk/instructions";

const PROGRAM_ID = new PublicKey("835WRKhFSAppy7p4QnFBkXJ6Mec3hAx3Jw2X15JKccyi")

async function main() {
  const runner = await FunctionRunner.create();

  const refreshOraclesIxn: TransactionInstruction = await generateUpdateIx(
    runner,
  );

  await runner.emit([refreshOraclesIxn]);
}

main();


async function generateUpdateIx(
    runner: FunctionRunner,
  ): Promise<TransactionInstruction> {
    const [oracle, bump] = PublicKey.findProgramAddressSync([Buffer.from("oracle")], PROGRAM_ID)

    const accounts: UpdateOracleAccounts = {
      oracle,
      function: runner.functionKey,
      enclaveSigner: runner.signer
    }

    const ix = updateOracle(accounts)
    return ix
  }