//! Program state
use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{program_pack::IsInitialized, pubkey::Pubkey},
};

/// Struct provided metadata of the related program
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct RelyingPartyData {
    /// Struct version, allows for upgrades to the program
    pub version: u8,
    /// The account allowed to update the data
    pub authority: Pubkey,
    /// The metadata of the related program
    pub related_program_data: RelatedProgramInfo,
}

/// Metadata of the some program to show for Vaccount
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, BorshSchema, PartialEq)]
pub struct RelatedProgramInfo {
    /// Name of the program to show in Vaccount
    pub name: String,
    /// Icon content identifier
    pub icon_cid: Vec<u8>,
    /// Domain name of the related program
    pub domain_name: String,
    /// Allowed redirect URI for Vaccount in program
    pub redirect_uri: Vec<String>,
}

impl RelatedProgramInfo {
    /// https://en.wikipedia.org/wiki/Domain_name#Domain_name_syntax
    pub const MAX_DOMAIN_LEN: u8 = 253;
    /// Is valid domain name
    pub fn is_valid_domain_name(domain_name: &str) -> bool {
        if domain_name.len() > Self::MAX_DOMAIN_LEN as usize {
            return false;
        }
        true
    }
}

impl RelyingPartyData {
    /// Version to fill in on new created accounts
    pub const CURRENT_VERSION: u8 = 1;
}

impl IsInitialized for RelyingPartyData {
    /// Is initialized
    fn is_initialized(&self) -> bool {
        self.version == Self::CURRENT_VERSION
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use cid::Cid;
    use std::convert::TryFrom;
    /// Version for tests
    pub const TEST_VERSION: u8 = 1;
    /// Pubkey for tests
    pub const TEST_AUTHORITY_PUBKEY: Pubkey = Pubkey::new_from_array([100; 32]);
    /// Pubkey for tests
    pub const TEST_RELATED_PROGRAM_PUBKEY: Pubkey = Pubkey::new_from_array([100; 32]);
    /// Related program name
    #[test]
    fn serialize_desialize_data() {
        let bs58_cid = "QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n";
        let cid = Cid::try_from(bs58_cid).unwrap().to_bytes();

        let program_data: RelatedProgramInfo = RelatedProgramInfo {
            name: String::from("test_program"),
            icon_cid: cid,
            domain_name: String::from("http://localhost:8989/"),
            redirect_uri: vec![
                "https://exzo.com/ru".to_string(),
                "https://wallet.exzo.com/".to_string(),
            ],
        };

        let relying_party_data: RelyingPartyData = RelyingPartyData {
            version: TEST_VERSION,
            authority: TEST_AUTHORITY_PUBKEY,
            related_program_data: program_data,
        };

        let packed = relying_party_data.try_to_vec().unwrap();
        let unpacked = RelyingPartyData::try_from_slice(packed.as_slice()).unwrap();

        assert_eq!(relying_party_data, unpacked);
    }
}
