import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IDL, OraclePoc } from "../target/types/oracle_poc";
import {
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  AttestationQueueAccount,
  FunctionAccount,
  SwitchboardProgram,
  TransactionObject,
} from "@switchboard-xyz/solana.js";
import { loadDefaultQueue } from "../scripts/utils";

describe("oracle-poc", () => {
  let program: anchor.Program<OraclePoc>;
  let switchboardProgram: SwitchboardProgram;
  let programStatePubkey: PublicKey;
  let attestationQueueAccount: AttestationQueueAccount;
  let functionAccount: FunctionAccount;
  let functionInit: TransactionObject;
  let oracle: PublicKey;

  const ORACLE_NAME = "TEST";
  const oracleBuffer = Buffer.alloc(16);
  oracleBuffer.fill(ORACLE_NAME, 0, Buffer.from(ORACLE_NAME).length);
  // console.log(oracleBuffer.buffer)

  before(async () => {
    program = new anchor.Program(
      IDL,
      new PublicKey("7zNxbvdozQr5zmg6fX3ZpZhWGtoCpUvpSxHXvC25gSWS"),
      provider
    );
    switchboardProgram = await SwitchboardProgram.fromProvider(provider);

    [programStatePubkey] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("ORACLEPOC")],
      program.programId
    );

    attestationQueueAccount = new AttestationQueueAccount(
      switchboardProgram,
      new PublicKey("2ie3JZfKcvsRLsJaP5fSo43gUo1vsurnUAtAgUdUAiDG")
    );
  });
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  // TODO: why is anchor workspace empty?

  it("Is initialized!", async () => {
    // Create the instructions to initialize our Switchboard Function
    [functionAccount, functionInit] =
      await attestationQueueAccount.createFunctionInstruction(
        provider.wallet.publicKey,
        {
          name: `${process.env.DOCKERHUB_CONTAINER_NAME}`,
          schedule: "15 * * * * *", // TODO: set a real schedule
          container: `${process.env.DOCKERHUB_ORGANIZATION}/${process.env.DOCKERHUB_CONTAINER_NAME}`,
          containerRegistry: "dockerhub",
          version: "latest",
        }
      );

    const signature = await program.methods
      .initialize()
      .accounts({
        program: programStatePubkey,
        authority: provider.wallet.publicKey,
        switchboardFunction: functionAccount.publicKey,
      })
      .signers([...functionInit.signers])
      .preInstructions([...functionInit.ixns])
      .rpc();

    console.log(`Initialize : ${signature}`);
  });

  it("Can add an oracle", async () => {
    [oracle] = PublicKey.findProgramAddressSync(
      [oracleBuffer],
      program.programId
    );

    const signature = await program.methods
      .addOracle({ name: ORACLE_NAME })
      .accounts({
        oracle,
        program: programStatePubkey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log(`Add Oracle: ${signature}`);
  });

  it("Can update an oracle", async () => {
    const signature = await program.methods
      .updateOracle({
        priceRaw: new anchor.BN(11),
        publishTime: new anchor.BN(25),
      })
      .accounts({
        oracle,
        program: programStatePubkey,
        switchboardFunction: functionAccount.publicKey,
        enclaveSigner: functionInit.payer,
      })
      .rpc();

    console.log(`Update Oracle: ${signature}`);
  });
});
