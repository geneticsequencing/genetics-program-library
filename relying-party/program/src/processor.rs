//! Program state processor

use crate::{
    borsh_utils::get_instance_packed_len,
    error::RelyingParty,
    id,
    instruction::RelyingPartyInstruction,
    state::{RelatedProgramInfo, RelyingPartyData},
};
use borsh::{BorshDeserialize, BorshSerialize};
use cid::Cid;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    hash::{Hash, Hasher},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use std::cmp::min;
use std::convert::TryFrom;

fn check_authority(authority_info: &AccountInfo, expected_authority: &Pubkey) -> ProgramResult {
    if expected_authority != authority_info.key {
        msg!("Incorrect RelyingParty authority provided");
        return Err(RelyingParty::IncorrectAuthority.into());
    }
    if !authority_info.is_signer {
        msg!("RelyingParty authority signature missing");
        return Err(ProgramError::MissingRequiredSignature);
    }

    Ok(())
}

fn get_redirect_uris_hash(program_redirect_uris: &[String]) -> Hash {
    let mut hasher = Hasher::default();
    for uri in program_redirect_uris.iter() {
        hasher.hash(uri.as_bytes());
    }
    hasher.result()
}

fn check_relying_party_address(
    relying_party_address: &Pubkey,
    program_name: &str,
    program_icon_cid: &str,
    program_domain_name: &str,
    program_redirect_uris: &[String],
    seed_nonce: &u8,
) -> ProgramResult {
    let relying_party_seed = [
        &program_name.as_bytes()[..min(32, program_name.len())],
        &program_icon_cid.as_bytes()[..min(32, program_icon_cid.len())],
        &program_domain_name.as_bytes()[..min(32, program_domain_name.len())],
        &get_redirect_uris_hash(program_redirect_uris).to_bytes()[..32],
        &[*seed_nonce],
    ];
    let expected_relying_party_address =
        Pubkey::create_program_address(&relying_party_seed, &id())?;

    if expected_relying_party_address != *relying_party_address {
        return Err(RelyingParty::InvalidRelyingPartyAddress.into());
    }

    Ok(())
}

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = RelyingPartyInstruction::try_from_slice(input)?;
    let account_info_iter = &mut accounts.iter();

    match instruction {
        RelyingPartyInstruction::Initialize {
            program_name,
            program_icon_cid,
            program_domain_name,
            program_redirect_uri,
            bump_seed_nonce,
        } => {
            msg!("RelyingPartyInstruction::Initialize");
            let icon_content_id =
                if let Ok(icon_content_id) = Cid::try_from(program_icon_cid.clone()) {
                    icon_content_id.to_bytes()
                } else {
                    return Err(RelyingParty::InvalidIconCID.into());
                };

            let relying_party_info = next_account_info(account_info_iter)?;
            let authority_info = next_account_info(account_info_iter)?;
            // Rent sysvar account
            let rent_info = next_account_info(account_info_iter)?;
            let rent = &Rent::from_account_info(rent_info)?;
            // System program account
            let system_program_info = next_account_info(account_info_iter)?;

            if !relying_party_info.data_is_empty() {
                return Err(ProgramError::InvalidAccountData);
            }

            check_relying_party_address(
                relying_party_info.key,
                &program_name,
                &program_icon_cid,
                &program_domain_name,
                &program_redirect_uri,
                &bump_seed_nonce,
            )?;

            let relying_party_account_data = RelyingPartyData {
                version: RelyingPartyData::CURRENT_VERSION,
                authority: *authority_info.key,
                related_program_data: RelatedProgramInfo {
                    name: program_name.clone(),
                    icon_cid: icon_content_id,
                    domain_name: program_domain_name.clone(),
                    redirect_uri: program_redirect_uri.clone(),
                },
            };

            // Fund the relying party with rent-exempt balance
            let required_relying_party_lamports =
                rent.minimum_balance(get_instance_packed_len(&relying_party_account_data).unwrap());
            if relying_party_info.lamports() < required_relying_party_lamports {
                return Err(ProgramError::AccountNotRentExempt);
            }

            let authority_relying_party_signature_seeds = [
                &program_name.as_bytes()[..min(32, program_name.len())],
                &program_icon_cid.as_bytes()[..min(32, program_icon_cid.len())],
                &program_domain_name.as_bytes()[..min(32, program_domain_name.len())],
                &get_redirect_uris_hash(&program_redirect_uri).to_bytes()[..32],
                &[bump_seed_nonce],
            ];
            let new_relying_party_signers = &[&authority_relying_party_signature_seeds[..]];

            // allocate space in vaccount
            invoke_signed(
                &system_instruction::allocate(
                    relying_party_info.key,
                    get_instance_packed_len(&relying_party_account_data).unwrap() as u64,
                ),
                &[relying_party_info.clone(), system_program_info.clone()],
                new_relying_party_signers,
            )?;

            // assign owner from system program to vaccount
            invoke_signed(
                &system_instruction::assign(relying_party_info.key, program_id),
                &[relying_party_info.clone(), system_program_info.clone()],
                new_relying_party_signers,
            )?;

            relying_party_account_data
                .serialize(&mut *relying_party_info.data.borrow_mut())
                .map_err(|e| e.into())
        }

        RelyingPartyInstruction::SetAuthority => {
            msg!("RelyingPartyInstruction::SetAuthority");
            let relying_party_info = next_account_info(account_info_iter)?;
            let authority_info = next_account_info(account_info_iter)?;
            let new_authority_info = next_account_info(account_info_iter)?;

            let mut relying_party_data =
                RelyingPartyData::try_from_slice(&relying_party_info.data.borrow())?;
            if !relying_party_data.is_initialized() {
                msg!("RelyingParty account not initialized");
                return Err(ProgramError::UninitializedAccount);
            }

            check_authority(authority_info, &relying_party_data.authority)?;

            relying_party_data.authority = *new_authority_info.key;
            relying_party_data
                .serialize(&mut *relying_party_info.data.borrow_mut())
                .map_err(|e| e.into())
        }

        RelyingPartyInstruction::CloseAccount => {
            msg!("RelyingPartyInstruction::CloseAccount");
            let relying_party_info = next_account_info(account_info_iter)?;
            let authority_info = next_account_info(account_info_iter)?;
            let destination_info = next_account_info(account_info_iter)?;

            let relying_party_data =
                RelyingPartyData::try_from_slice(&relying_party_info.data.borrow())?;
            if !relying_party_data.is_initialized() {
                msg!("RelyingParty not initialized");
                return Err(ProgramError::UninitializedAccount);
            }

            check_authority(authority_info, &relying_party_data.authority)?;

            let relying_party_data_lamports = relying_party_info.lamports();
            **relying_party_info.lamports.borrow_mut() = 0;
            **destination_info.lamports.borrow_mut() += relying_party_data_lamports;

            Ok(())
        }
    }
}
