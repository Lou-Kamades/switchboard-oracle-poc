import { PublicKey, Connection } from "@solana/web3.js"
import BN from "bn.js" // eslint-disable-line @typescript-eslint/no-unused-vars
import * as borsh from "@coral-xyz/borsh" // eslint-disable-line @typescript-eslint/no-unused-vars
import { PROGRAM_ID } from "../programId"

export interface OracleDataFields {
  oracleTimestamp: BN
  price: BN
  bump: number
}

export interface OracleDataJSON {
  oracleTimestamp: string
  price: string
  bump: number
}

export class OracleData {
  readonly oracleTimestamp: BN
  readonly price: BN
  readonly bump: number

  static readonly discriminator = Buffer.from([
    26, 131, 25, 110, 6, 141, 10, 37,
  ])

  static readonly layout = borsh.struct([
    borsh.i64("oracleTimestamp"),
    borsh.i128("price"),
    borsh.u8("bump"),
  ])

  constructor(fields: OracleDataFields) {
    this.oracleTimestamp = fields.oracleTimestamp
    this.price = fields.price
    this.bump = fields.bump
  }

  static async fetch(
    c: Connection,
    address: PublicKey,
    programId: PublicKey = PROGRAM_ID
  ): Promise<OracleData | null> {
    const info = await c.getAccountInfo(address)

    if (info === null) {
      return null
    }
    if (!info.owner.equals(programId)) {
      throw new Error("account doesn't belong to this program")
    }

    return this.decode(info.data)
  }

  static async fetchMultiple(
    c: Connection,
    addresses: PublicKey[],
    programId: PublicKey = PROGRAM_ID
  ): Promise<Array<OracleData | null>> {
    const infos = await c.getMultipleAccountsInfo(addresses)

    return infos.map((info) => {
      if (info === null) {
        return null
      }
      if (!info.owner.equals(programId)) {
        throw new Error("account doesn't belong to this program")
      }

      return this.decode(info.data)
    })
  }

  static decode(data: Buffer): OracleData {
    if (!data.slice(0, 8).equals(OracleData.discriminator)) {
      throw new Error("invalid account discriminator")
    }

    const dec = OracleData.layout.decode(data.slice(8))

    return new OracleData({
      oracleTimestamp: dec.oracleTimestamp,
      price: dec.price,
      bump: dec.bump,
    })
  }

  toJSON(): OracleDataJSON {
    return {
      oracleTimestamp: this.oracleTimestamp.toString(),
      price: this.price.toString(),
      bump: this.bump,
    }
  }

  static fromJSON(obj: OracleDataJSON): OracleData {
    return new OracleData({
      oracleTimestamp: new BN(obj.oracleTimestamp),
      price: new BN(obj.price),
      bump: obj.bump,
    })
  }
}
