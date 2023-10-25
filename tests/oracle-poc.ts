import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IDL, OraclePoc } from "../target/types/oracle_poc";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { MINT_SIZE, TOKEN_PROGRAM_ID, createInitializeMintInstruction, createMint, getMinimumBalanceForRentExemptMint } from '@solana/spl-token'
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
  let mintPubkey: PublicKey;
  let oracleContainer: PublicKey;

  const ORACLE_NAME = "TEST";
  const oracleBuffer = Buffer.alloc(16);
  oracleBuffer.fill(ORACLE_NAME, 0, Buffer.from(ORACLE_NAME).length);
  // console.log(oracleBuffer.buffer)

  before(async () => {
    program = new anchor.Program(
      IDL,
      new PublicKey("GknYjbiQABncTa8JwStdHRX1t1UZArjdAoaRTrccfhdR"),
      provider
    );
    switchboardProgram = await SwitchboardProgram.fromProvider(provider);

    [programStatePubkey] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("ORACLEPOC")],
      program.programId
    );
    [oracleContainer] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("ORACLE")],
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
        oracleContainer: oracleContainer,
        authority: provider.wallet.publicKey,
        switchboardFunction: functionAccount.publicKey,
      })
      .signers([...functionInit.signers])
      .preInstructions([...functionInit.ixns])
      .rpc();

    console.log(`Initialize : ${signature}`);
  });

  it ("Inits a mint for the Program", async () => {
    const mint = Keypair.generate();
    mintPubkey = mint.publicKey

    let tx = new Transaction().add(
      // create mint account
      SystemProgram.createAccount({
        fromPubkey: provider.wallet.publicKey,
        newAccountPubkey: mint.publicKey,
        space: MINT_SIZE,
        lamports: await getMinimumBalanceForRentExemptMint(provider.connection),
        programId: TOKEN_PROGRAM_ID,
      }),
      // init mint account
      createInitializeMintInstruction(
        mint.publicKey, // mint pubkey
        8, // decimals
         provider.wallet.publicKey, // mint authority
         provider.wallet.publicKey // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
      )
    );

    await provider.sendAndConfirm(tx, [mint])
  })

  it("Can add an oracle", async () => {
    const signature = await program.methods
      .addOracle({ name: ORACLE_NAME, quoteSizeUsdcNative: new anchor.BN(2_000_000) })
      .accounts({
        oracleContainer,
        oracleMint: mintPubkey,
        program: programStatePubkey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log(`Add Oracle: ${signature}`);
  });

  it("Can update an oracle's price", async () => {
    const signature = await program.methods
      .updateOracle({
        price: 11.1,
        oracleName: ORACLE_NAME,
      })
      .accounts({
        oracleContainer,
        program: programStatePubkey,
        switchboardFunction: functionAccount.publicKey,
        enclaveSigner: functionInit.payer,
      })
      .rpc();

    console.log(`Update Oracle: ${signature}`);
  });

  it("Can update an oracle's quote size", async () => {
    const signature = await program.methods
      .setOracleQuoteSize({
        oracleName: ORACLE_NAME,
        quoteSizeUsdcNative: new anchor.BN(3_000_000)
      })
      .accounts({
        oracleContainer,
        program: programStatePubkey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    console.log(`Set Oracle Quote Size: ${signature}`);
  });
});
