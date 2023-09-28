import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface SaveDataAccounts {
  function: PublicKey
  oracle: PublicKey
  enclaveSigner: PublicKey
}

export function saveData(
  accounts: SaveDataAccounts,
  programId: PublicKey = PROGRAM_ID
) {
  const keys: Array<AccountMeta> = [
    { pubkey: accounts.function, isSigner: false, isWritable: false },
    { pubkey: accounts.oracle, isSigner: false, isWritable: true },
    { pubkey: accounts.enclaveSigner, isSigner: true, isWritable: false },
  ]
  const identifier = Buffer.from([227, 228, 223, 152, 188, 62, 31, 151])
  const data = identifier
  const ix = new TransactionInstruction({ keys, programId, data })
  return ix
}
