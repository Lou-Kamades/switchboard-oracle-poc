import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface UpdateOracleAccounts {
  function: PublicKey
  oracle: PublicKey
  enclaveSigner: PublicKey
}

export function updateOracle(
  accounts: UpdateOracleAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.function, isSigner: false, isWritable: false },
    { pubkey: accounts.oracle, isSigner: false, isWritable: true },
    { pubkey: accounts.enclaveSigner, isSigner: true, isWritable: false },
  ]
  const identifier = Buffer.from([112, 41, 209, 18, 248, 226, 252, 188])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
