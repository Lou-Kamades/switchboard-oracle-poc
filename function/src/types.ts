export type FatOracle = {
  "version": "0.1.0",
  "name": "fat_oracle",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "oracle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "saveData",
      "accounts": [
        {
          "name": "function",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "enclaveSigner",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "oracleData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "oracleTimestamp",
            "type": "i64"
          },
          {
            "name": "price",
            "type": "i128"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};

export const IDL: FatOracle = {
  "version": "0.1.0",
  "name": "fat_oracle",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "oracle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "saveData",
      "accounts": [
        {
          "name": "function",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "oracle",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "enclaveSigner",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "oracleData",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "oracleTimestamp",
            "type": "i64"
          },
          {
            "name": "price",
            "type": "i128"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
