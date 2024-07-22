/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/journal.json`.
 */
export type Journal = {
  "address": "7AGmMcgd1SjoMsCcXAAYwRgB9ihCyM8cZqjsUqriNRQt",
  "metadata": {
    "name": "journal",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "docs": [
    "The main program module for the journal.",
    "The `#[program]` attribute macro defines the entry points for the Solana program."
  ],
  "instructions": [
    {
      "name": "createJournalEntry",
      "docs": [
        "Creates a new journal entry.",
        "",
        "# Arguments",
        "",
        "* `ctx` - The context containing the accounts involved in the transaction.",
        "* `title` - The title of the journal entry.",
        "* `message` - The message of the journal entry.",
        "",
        "# Returns",
        "",
        "* `Result<()>` - Returns an empty result on success.",
        "",
        "This function initializes a new journal entry account with the provided title and message.",
        "It logs the creation of the entry and assigns the owner's public key to the entry."
      ],
      "discriminator": [
        48,
        65,
        201,
        186,
        25,
        41,
        127,
        0
      ],
      "accounts": [
        {
          "name": "journalEntry",
          "docs": [
            "The account to be created or initialized for the journal entry.",
            "",
            "- `init_if_needed`: Initializes the account if it doesn't already exist.",
            "- `seeds`: A unique identifier for the account, derived from the title and owner's public key.",
            "- `bump`: A nonce used to ensure the uniqueness of the derived address.",
            "- `payer`: The account that will pay for the account creation.",
            "- `space`: The amount of space to allocate for the account."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "title"
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "docs": [
            "The signer of the transaction.",
            "This account must sign the transaction to authorize it."
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "docs": [
            "The system program required for account creation.",
            "This is a built-in program that provides basic account management functionalities."
          ],
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "title",
          "type": "string"
        },
        {
          "name": "message",
          "type": "string"
        }
      ]
    },
    {
      "name": "deleteJournalEntry",
      "docs": [
        "Deletes an existing journal entry.",
        "",
        "# Arguments",
        "",
        "* `ctx` - The context containing the accounts involved in the transaction.",
        "* `title` - The title of the journal entry to be deleted.",
        "",
        "# Returns",
        "",
        "* `Result<()>` - Returns an empty result on success.",
        "",
        "This function deletes an existing journal entry account.",
        "It logs the deletion of the entry."
      ],
      "discriminator": [
        156,
        50,
        93,
        5,
        157,
        97,
        188,
        114
      ],
      "accounts": [
        {
          "name": "journalEntry",
          "docs": [
            "The account to be deleted for the journal entry.",
            "",
            "- `mut`: The account is mutable, meaning it can be modified.",
            "- `seeds`: A unique identifier for the account, derived from the title and owner's public key.",
            "- `bump`: A nonce used to ensure the uniqueness of the derived address.",
            "- `close`: Closes the account and transfers the remaining lamports to the specified account."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "title"
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "docs": [
            "The signer of the transaction.",
            "This account must sign the transaction to authorize it."
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "docs": [
            "The system program required for account closure.",
            "This is a built-in program that provides basic account management functionalities."
          ],
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "title",
          "type": "string"
        }
      ]
    },
    {
      "name": "updateJournalEntry",
      "docs": [
        "Updates an existing journal entry.",
        "",
        "# Arguments",
        "",
        "* `ctx` - The context containing the accounts involved in the transaction.",
        "* `title` - The new title of the journal entry.",
        "* `message` - The new message of the journal entry.",
        "",
        "# Returns",
        "",
        "* `Result<()>` - Returns an empty result on success.",
        "",
        "This function updates the message of an existing journal entry account with the provided message.",
        "It logs the update of the entry."
      ],
      "discriminator": [
        113,
        164,
        49,
        62,
        43,
        83,
        194,
        172
      ],
      "accounts": [
        {
          "name": "journalEntry",
          "docs": [
            "The account to be updated for the journal entry.",
            "",
            "- `mut`: The account is mutable, meaning it can be modified.",
            "- `seeds`: A unique identifier for the account, derived from the title and owner's public key.",
            "- `bump`: A nonce used to ensure the uniqueness of the derived address.",
            "- `realloc`: Reallocates the account with the new size.",
            "- `realloc::payer`: The account that will pay for the reallocation.",
            "- `realloc::zero`: Ensures the newly allocated space is zeroed out."
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "arg",
                "path": "title"
              },
              {
                "kind": "account",
                "path": "owner"
              }
            ]
          }
        },
        {
          "name": "owner",
          "docs": [
            "The signer of the transaction.",
            "This account must sign the transaction to authorize it."
          ],
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "docs": [
            "The system program required for account reallocation.",
            "This is a built-in program that provides basic account management functionalities."
          ],
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "title",
          "type": "string"
        },
        {
          "name": "message",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "journalEntryState",
      "discriminator": [
        113,
        86,
        110,
        124,
        140,
        14,
        58,
        66
      ]
    }
  ],
  "types": [
    {
      "name": "journalEntryState",
      "docs": [
        "Represents the state of a journal entry.",
        "The `#[account]` attribute macro defines a struct that will be stored on-chain.",
        "The `#[derive(InitSpace)]` attribute macro is used to initialize the account with a space of 8 bytes."
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "docs": [
              "The public key of the owner of the journal entry."
            ],
            "type": "pubkey"
          },
          {
            "name": "title",
            "docs": [
              "The title of the journal entry. Maximum length is 50 characters."
            ],
            "type": "string"
          },
          {
            "name": "message",
            "docs": [
              "The message of the journal entry. Maximum length is 1000 characters."
            ],
            "type": "string"
          }
        ]
      }
    }
  ]
};
