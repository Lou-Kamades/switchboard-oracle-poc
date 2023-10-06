import * as anchor from "@coral-xyz/anchor";
import { Program, } from "@coral-xyz/anchor";
import { FatOracle } from "../target/types/oracle_poc";
import { PublicKey, SystemProgram, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import { InitializeAccounts, UpdateOracleAccounts, initialize, updateOracle } from "../function-ts/src/sdk/instructions";
import { FunctionAccount } from "@switchboard-xyz/solana.js";

describe("oracle-poc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FatOracle as Program<FatOracle>;

  it("Is initialized!", async () => {
    const provider = anchor.getProvider()
    const tx = new Transaction()
    const [oracle, bump] = PublicKey.findProgramAddressSync([Buffer.from("oracle")], program.programId)

    const accounts: InitializeAccounts = {
        oracle,
        payer: provider.publicKey,
        systemProgram: SystemProgram.programId
    }

    const ix = initialize(accounts)
    tx.add(ix)

    const sig = await provider.sendAndConfirm( tx, [], {skipPreflight: true})
    console.log(sig)
  });

  // it ("Can update oracle", async () => {

  //   const provider = anchor.getProvider()
  //   const tx = new Transaction()
  //   const [oracle, bump] = PublicKey.findProgramAddressSync([Buffer.from("oracle")], program.programId)

  //   const accounts: UpdateOracleAccounts = {
  //       function: 
  //       oracle,
  //       enclaveSigner:
  //   }

  //   const ix = updateOracle(accounts)
  //   tx.add(ix)

  //   const sig = await provider.sendAndConfirm( tx, [], {skipPreflight: true})
  //   console.log(sig)

  // })
});
