import { Program } from "@coral-xyz/anchor";
import idl from "./idl.json";
import { FunctionRunner } from "@switchboard-xyz/solana.js/functions";
import { FatOracle } from "./types";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";

async function main() {
  const runner = new FunctionRunner();

  const program: Program<FatOracle> = new Program(
    JSON.parse(JSON.stringify(idl)),
    "3NKUtPKboaQN4MwY3nyULBesFaW7hHsXFrBTVjbn2nBr",
    runner.provider
  );

  const refreshOraclesIxn: TransactionInstruction = await generateUpdateIx(
    runner,
    program
  );

  await runner.emit([refreshOraclesIxn]);
}

// run switchboard function
main();



async function generateUpdateIx(
    runner: FunctionRunner,
    program: Program<FatOracle>
  ): Promise<TransactionInstruction> {
    return await program.methods
      .updateOracle()
      .accounts({
        switchboardFunction: runner.functionKey,
        oracle: PublicKey.findProgramAddressSync(
          [Buffer.from("oracle")],
          program.programId
        )[0],
        enclaveSigner: runner.signer,
      })
      .instruction();
  }