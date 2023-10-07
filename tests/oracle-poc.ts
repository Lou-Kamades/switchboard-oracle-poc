import * as anchor from "@coral-xyz/anchor";
import { Program, } from "@coral-xyz/anchor";
import { OraclePoc } from "../target/types/oracle_poc";
import { PublicKey, SystemProgram, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import { FunctionAccount } from "@switchboard-xyz/solana.js";

describe("oracle-poc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.oraclePoc as Program<OraclePoc>;

  it("Is initialized!", async () => {
    // TODO
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
