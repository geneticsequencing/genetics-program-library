//! Program instructions

use crate::id;
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar::rent,
};

/// Instructions supported by the program
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum RelyingPartyInstruction {
    /// Create a new relying party account
    /// RelyingPartyProgram contain meta inforamation about some dapp that needed for VaccountProgram.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable]` RelyingParty account, must be uninitialized
    /// 1. `[]` RelyingParty authority
    /// 2. `[]` Related program to the RelyingParty
    Initialize {
        /// Dapp name to show in Vaccount
        #[allow(dead_code)] // but it's not
        program_name: String,
        /// Dapp icon content identifier
        #[allow(dead_code)] // but it's not
        program_icon_cid: String,
        /// Domain name of the Dapp
        #[allow(dead_code)] // but it's not
        program_domain_name: String,
        /// Allowed redirect URI
        #[allow(dead_code)] // but it's not
        program_redirect_uri: Vec<String>,
        /// Nonce with RelyingParty address was genereted
        #[allow(dead_code)] // but it's not
        bump_seed_nonce: u8,
    },

    /// Update the authority of the provided RelyingParty account
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable]` RelyingParty account, must be previously initialized
    /// 1. `[signer]` Current RelyingParty authority
    /// 2. `[]` New RelyingParty authority
    SetAuthority,

    /// Close the provided RelyingParty account, draining lamports to recipient account
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable]` RelyingParty account, must be previously initialized
    /// 1. `[signer]` RelyingParty authority
    /// 2. `[]` Receiver of account lamports
    CloseAccount,
}

/// Create a `RelyingPartyInstruction::Initialize` instruction
pub fn initialize(
    relying_party_account: &Pubkey,
    authority: &Pubkey,
    program_name: String,
    program_icon_cid: String,
    program_domain_name: String,
    program_redirect_uri: Vec<String>,
    bump_seed_nonce: u8,
) -> Instruction {
    Instruction::new_with_borsh(
        id(),
        &RelyingPartyInstruction::Initialize {
            program_name,
            program_icon_cid,
            program_domain_name,
            program_redirect_uri,
            bump_seed_nonce,
        },
        vec![
            AccountMeta::new(*relying_party_account, false),
            AccountMeta::new_readonly(*authority, false),
            AccountMeta::new_readonly(rent::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
    )
}

/// Create a `RelyingPartyInstruction::SetAuthority` instruction
pub fn set_authority(
    relying_party_account: &Pubkey,
    signer: &Pubkey,
    new_authority: &Pubkey,
) -> Instruction {
    Instruction::new_with_borsh(
        id(),
        &RelyingPartyInstruction::SetAuthority,
        vec![
            AccountMeta::new(*relying_party_account, false),
            AccountMeta::new_readonly(*signer, true),
            AccountMeta::new_readonly(*new_authority, false),
        ],
    )
}

/// Create a `RelyingPartyInstruction::CloseAccount` instruction
pub fn close_account(
    relying_party_account: &Pubkey,
    signer: &Pubkey,
    receiver: &Pubkey,
) -> Instruction {
    Instruction::new_with_borsh(
        id(),
        &RelyingPartyInstruction::CloseAccount,
        vec![
            AccountMeta::new(*relying_party_account, false),
            AccountMeta::new_readonly(*signer, true),
            AccountMeta::new(*receiver, false),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::program_error::ProgramError;

    #[test]
    fn serialize_initialize() {
        let instruction = RelyingPartyInstruction::Initialize {
            program_name: String::from("test_program"),
            program_icon_cid: "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n".to_string(),
            program_domain_name: String::from("http://localhost:8989/"),
            program_redirect_uri: vec![
                "https://exzo.com/ru".to_string(),
                "https://wallet.exzo.com/".to_string(),
            ],
            bump_seed_nonce: 199,
        };
        let expected = vec![
            0, 12, 0, 0, 0, 116, 101, 115, 116, 95, 112, 114, 111, 103, 114, 97, 109, 46, 0, 0, 0,
            81, 109, 100, 102, 84, 98, 66, 113, 66, 80, 81, 55, 86, 78, 120, 90, 69, 89, 69, 106,
            49, 52, 86, 109, 82, 117, 90, 66, 107, 113, 70, 98, 105, 119, 82, 101, 111, 103, 74,
            103, 83, 49, 122, 82, 49, 110, 22, 0, 0, 0, 104, 116, 116, 112, 58, 47, 47, 108, 111,
            99, 97, 108, 104, 111, 115, 116, 58, 56, 57, 56, 57, 47, 2, 0, 0, 0, 20, 0, 0, 0, 104,
            116, 116, 112, 115, 58, 47, 47, 118, 101, 108, 97, 115, 46, 99, 111, 109, 47, 114, 117,
            25, 0, 0, 0, 104, 116, 116, 112, 115, 58, 47, 47, 119, 97, 108, 108, 101, 116, 46, 118,
            101, 108, 97, 115, 46, 99, 111, 109, 47, 199,
        ];
        assert_eq!(instruction.try_to_vec().unwrap(), expected);
        assert_eq!(
            RelyingPartyInstruction::try_from_slice(&expected).unwrap(),
            instruction
        );
    }

    #[test]
    fn serialize_set_authority() {
        let instruction = RelyingPartyInstruction::SetAuthority;
        let expected = vec![1];
        assert_eq!(instruction.try_to_vec().unwrap(), expected);
        assert_eq!(
            RelyingPartyInstruction::try_from_slice(&expected).unwrap(),
            instruction
        );
    }

    #[test]
    fn serialize_close_account() {
        let instruction = RelyingPartyInstruction::CloseAccount;
        let expected = vec![2];
        assert_eq!(instruction.try_to_vec().unwrap(), expected);
        assert_eq!(
            RelyingPartyInstruction::try_from_slice(&expected).unwrap(),
            instruction
        );
    }

    #[test]
    fn deserialize_invalid_instruction() {
        let mut expected = vec![12];
        expected.append(&mut "TEST_DATA".try_to_vec().unwrap());
        let err: ProgramError = RelyingPartyInstruction::try_from_slice(&expected)
            .unwrap_err()
            .into();
        assert!(matches!(err, ProgramError::BorshIoError(_)));
    }
}
