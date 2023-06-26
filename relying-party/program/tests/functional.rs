// // Mark this test as BPF-only due to current `ProgramTest` limitations when CPIing into the system program
// // #![cfg(feature = "test-bpf")]

// TODO: Uncoment when will be fixed: https://github.com/solana-labs/solana/blob/6d11d5dd9f04735be5a2c7aa4a4e8517a4417b88/program-test/src/lib.rs#L361
// use {
//     borsh::{BorshDeserialize, BorshSerialize, BorshSchema},
//     solana_program::{
//         borsh::get_packed_len,
//         instruction::{AccountMeta, Instruction, InstructionError},
//         pubkey::Pubkey,
//         rent::Rent,
//         system_instruction,
//         hash::{hashv, Hasher, Hash},
//     },
//     solana_program_test::*,
//     solana_sdk::{
//         signature::{Keypair, Signer},
//         transaction::{Transaction, TransactionError},
//         transport,
//     },
//     vpl_relying_party::{
//         error::RelyingParty,
//         id, instruction,
//         processor::process_instruction,
//         state::{RelyingPartyData, RelatedProgramInfo},
//         borsh_utils::get_instance_packed_len,
//     },
//     std::{
//         convert::TryInto,
//         cmp::min,
//     },
// };

// fn program_test() -> ProgramTest {
//     ProgramTest::new("vpl-relying-party", id(), processor!(process_instruction))
// }

// fn get_relying_party_address(
//     program_name: &String,
//     program_icon_cid: [u8; 32],
//     program_domain_name: &String,
//     program_redirect_uri: &Vec<String>,
// ) -> (Pubkey, u8) {
//     let mut hasher = Hasher::default();
//     for uri in program_redirect_uri.iter() {
//         hasher.hash(uri.as_bytes());
//     }
//     let program_redirect_uris_hash = hasher.result();

//     Pubkey::find_program_address(
//         &[
//             &program_name.as_bytes()[..min(32, program_name.len() - 1)],
//             &program_icon_cid[..min(32, program_icon_cid.len() - 1)],
//             &program_domain_name.as_bytes()[..min(32, program_domain_name.len() - 1)],
//             &program_redirect_uris_hash.to_bytes()[..32],
//         ],
//         &id(),
//     )
// }

// async fn initialize_relying_party_account(
//     context: &mut ProgramTestContext,
//     relying_party: &Pubkey,
//     authority: &Keypair,
//     related_program: &Pubkey,
//     program_name: &String,
//     program_icon_cid: [u8; 32],
//     program_domain_name: &String,
//     program_redirect_uri: &Vec<String>,
//     bump_seed_nonce: u8,
//     space: usize
// ) -> transport::Result<()> {
//     let transaction = Transaction::new_signed_with_payer(
//         &[
//             system_instruction::transfer(
//                 &context.payer.pubkey(),
//                 &relying_party,
//                 1.max(Rent::default().minimum_balance(space)),
//             ),
//             instruction::initialize(
//                 &relying_party,
//                 &authority.pubkey(),
//                 &related_program,
//                 program_name.clone(),
//                 program_icon_cid,
//                 program_domain_name.to_string(),
//                 program_redirect_uri.clone(),
//                 bump_seed_nonce,
//             ),
//         ],
//         Some(&context.payer.pubkey()),
//         &[&context.payer, authority],
//         context.last_blockhash,
//     );
//     context.banks_client.process_transaction(transaction).await
// }

// #[tokio::test]
// async fn initialize_success() {
//     let mut context = program_test().start_with_context().await;

//     let program_name: String = String::from("test_program");
//     let program_icon_cid: [u8; 32] = "d2a84f4b8b650937ec8f73cd8be2c74add5a911ba64df27458ed8229da804a26".as_bytes().try_into().unwrap();
//     let program_domain_name: String = String::from("http://localhost:8989/");
//     let program_redirect_uri: Vec<String> = vec![
//         "https://exzo.com/ru".to_string(),
//         "https://wallet.exzo.com/".to_string(),
//     ];

//     let (relying_party_address, bump_seed_nonce) = get_relying_party_address(
//         &program_name,
//         program_icon_cid,
//         &program_domain_name,
//         &program_redirect_uri,
//     );

//     let authority = Keypair::new();
//     let related_program = Keypair::new();

//     let related_program_data: RelatedProgramInfo = RelatedProgramInfo {
//         name: program_name.clone(),
//         icon_cid: program_icon_cid,
//         domain_name: program_domain_name.clone(),
//         redirect_uri: program_redirect_uri.clone(),
//     };

//     let relying_party_data: RelyingPartyData = RelyingPartyData {
//         version: 1,
//         authority: authority.pubkey(),
//         related_program: related_program.pubkey(),
//         related_program_data: related_program_data,
//     };

//     initialize_relying_party_account(
//         &mut context,
//         &relying_party_address,
//         &authority,
//         &related_program.pubkey(),
//         &program_name,
//         program_icon_cid,
//         &program_domain_name,
//         &program_redirect_uri,
//         bump_seed_nonce,
//         get_instance_packed_len(&relying_party_data).unwrap(),
//     )
//     .await
//     .unwrap();
//     let relying_party_address_data = context
//         .banks_client
//         .get_account_data_with_borsh::<RelyingPartyData>(relying_party_address)
//         .await
//         .unwrap();

//     let related_program_data = RelatedProgramInfo {
//         name: program_name,
//         icon_cid: program_icon_cid,
//         domain_name: program_domain_name,
//         redirect_uri: program_redirect_uri,
//     };

//     assert_eq!(relying_party_address_data.authority, authority.pubkey());
//     assert_eq!(relying_party_address_data.version, RelyingPartyData::CURRENT_VERSION);
//     assert_eq!(relying_party_address_data.related_program, related_program.pubkey());
//     assert_eq!(relying_party_address_data.related_program_data, related_program_data);
// }

// #[tokio::test]
// async fn initialize_twice_fail() {
//     let mut context = program_test().start_with_context().await;

//     let program_name: String = String::from("test_program");
//     let program_icon_cid: [u8; 32] = "d2a84f4b8b650937ec8f73cd8be2c74add5a911ba64df27458ed8229da804a26".as_bytes().try_into().unwrap();
//     let program_domain_name: String = String::from("http://localhost:8989/");
//     let program_redirect_uri: Vec<String> = vec![
//         "https://exzo.com/ru".to_string(),
//         "https://wallet.exzo.com/".to_string(),
//     ];

//     let (relying_party_address, bump_seed_nonce) = get_relying_party_address(
//         &program_name,
//         program_icon_cid,
//         &program_domain_name,
//         &program_redirect_uri,
//     );

//     let authority = Keypair::new();
//     let related_program = Keypair::new();

//     let related_program_data: RelatedProgramInfo = RelatedProgramInfo {
//         name: program_name.clone(),
//         icon_cid: program_icon_cid,
//         domain_name: program_domain_name.clone(),
//         redirect_uri: program_redirect_uri.clone(),
//     };

//     let relying_party_data: RelyingPartyData = RelyingPartyData {
//         version: 1,
//         authority: authority.pubkey(),
//         related_program: related_program.pubkey(),
//         related_program_data: related_program_data,
//     };

//     initialize_relying_party_account(
//         &mut context,
//         &relying_party_address,
//         &authority,
//         &related_program.pubkey(),
//         &program_name,
//         program_icon_cid,
//         &program_domain_name,
//         &program_redirect_uri,
//         bump_seed_nonce,
//         get_instance_packed_len(&relying_party_data).unwrap(),
//     )
//     .await
//     .unwrap();

//     let transaction = Transaction::new_signed_with_payer(
//         &[
//             system_instruction::transfer(
//                 &context.payer.pubkey(),
//                 &relying_party_address,
//                 1.max(Rent::default().minimum_balance(get_instance_packed_len(&relying_party_data).unwrap())),
//             ),
//             instruction::initialize(
//                 &relying_party_address,
//                 &authority.pubkey(),
//                 &related_program.pubkey(),
//                 program_name.clone(),
//                 program_icon_cid,
//                 program_domain_name.to_string(),
//                 program_redirect_uri.clone(),
//                 bump_seed_nonce,
//             ),
//         ],
//         Some(&context.payer.pubkey()),
//         &[&context.payer, &authority],
//         context.last_blockhash,
//     );

//     assert_eq!(
//         context
//             .banks_client
//             .process_transaction(transaction)
//             .await
//             .unwrap_err()
//             .unwrap(),
//         TransactionError::InstructionError(0, InstructionError::InvalidAccountData)
//     );
// }

// #[tokio::test]
// async fn close_account_success() {
//     let mut context = program_test().start_with_context().await;

//     let program_name: String = String::from("test_program");
//     let program_icon_cid: [u8; 32] = "d2a84f4b8b650937ec8f73cd8be2c74add5a911ba64df27458ed8229da804a26".as_bytes().try_into().unwrap();
//     let program_domain_name: String = String::from("http://localhost:8989/");
//     let program_redirect_uri: Vec<String> = vec![
//         "https://exzo.com/ru".to_string(),
//         "https://wallet.exzo.com/".to_string(),
//     ];

//     let (relying_party_address, bump_seed_nonce) = get_relying_party_address(
//         &program_name,
//         program_icon_cid,
//         &program_domain_name,
//         &program_redirect_uri,
//     );

//     let authority = Keypair::new();
//     let related_program = Keypair::new();
//     let recipient = Pubkey::new_unique();

//     let related_program_data: RelatedProgramInfo = RelatedProgramInfo {
//         name: program_name.clone(),
//         icon_cid: program_icon_cid,
//         domain_name: program_domain_name.clone(),
//         redirect_uri: program_redirect_uri.clone(),
//     };

//     let relying_party_data: RelyingPartyData = RelyingPartyData {
//         version: 1,
//         authority: authority.pubkey(),
//         related_program: related_program.pubkey(),
//         related_program_data: related_program_data,
//     };

//     initialize_relying_party_account(
//         &mut context,
//         &relying_party_address,
//         &authority,
//         &related_program.pubkey(),
//         &program_name,
//         program_icon_cid,
//         &program_domain_name,
//         &program_redirect_uri,
//         bump_seed_nonce,
//         get_instance_packed_len(&relying_party_data).unwrap(),
//     )
//     .await
//     .unwrap();

//     let transaction = Transaction::new_signed_with_payer(
//         &[instruction::close_account(
//             &relying_party_address,
//             &authority.pubkey(),
//             &recipient,
//         )],
//         Some(&context.payer.pubkey()),
//         &[&context.payer, &authority],
//         context.last_blockhash,
//     );
//     context
//         .banks_client
//         .process_transaction(transaction)
//         .await
//         .unwrap();

//     let account = context
//         .banks_client
//         .get_account(recipient)
//         .await
//         .unwrap()
//         .unwrap();
//     assert_eq!(
//         account.lamports,
//         1.max(Rent::default().minimum_balance(get_instance_packed_len(&relying_party_data).unwrap()))
//     );
// }

// #[tokio::test]
// async fn close_account_fail_wrong_authority() {
//     let mut context = program_test().start_with_context().await;

//     let program_name: String = String::from("test_program");
//     let program_icon_cid: [u8; 32] = "d2a84f4b8b650937ec8f73cd8be2c74add5a911ba64df27458ed8229da804a26".as_bytes().try_into().unwrap();
//     let program_domain_name: String = String::from("http://localhost:8989/");
//     let program_redirect_uri: Vec<String> = vec![
//         "https://exzo.com/ru".to_string(),
//         "https://wallet.exzo.com/".to_string(),
//     ];

//     let (relying_party_address, bump_seed_nonce) = get_relying_party_address(
//         &program_name,
//         program_icon_cid,
//         &program_domain_name,
//         &program_redirect_uri,
//     );

//     let authority = Keypair::new();
//     let wrong_authority = Keypair::new();
//     let related_program = Keypair::new();
//     let recipient = Pubkey::new_unique();

//     let related_program_data: RelatedProgramInfo = RelatedProgramInfo {
//         name: program_name.clone(),
//         icon_cid: program_icon_cid,
//         domain_name: program_domain_name.clone(),
//         redirect_uri: program_redirect_uri.clone(),
//     };

//     let relying_party_data: RelyingPartyData = RelyingPartyData {
//         version: 1,
//         authority: authority.pubkey(),
//         related_program: related_program.pubkey(),
//         related_program_data: related_program_data,
//     };

//     initialize_relying_party_account(
//         &mut context,
//         &relying_party_address,
//         &authority,
//         &related_program.pubkey(),
//         &program_name,
//         program_icon_cid,
//         &program_domain_name,
//         &program_redirect_uri,
//         bump_seed_nonce,
//         get_instance_packed_len(&relying_party_data).unwrap(),
//     )
//     .await
//     .unwrap();

//     let transaction = Transaction::new_signed_with_payer(
//         &[instruction::close_account(
//             &relying_party_address,
//             &wrong_authority.pubkey(),
//             &recipient,
//         )],
//         Some(&context.payer.pubkey()),
//         &[&context.payer, &authority],
//         context.last_blockhash,
//     );

//     assert_eq!(
//         context
//             .banks_client
//             .process_transaction(transaction)
//             .await
//             .unwrap_err()
//             .unwrap(),
//         TransactionError::InstructionError(
//             0,
//             InstructionError::Custom(RelyingParty::IncorrectAuthority as u32)
//         )
//     );
// }

// #[tokio::test]
// async fn close_account_fail_unsigned() {
//     let mut context = program_test().start_with_context().await;

//     let program_name: String = String::from("test_program");
//     let program_icon_cid: [u8; 32] = "d2a84f4b8b650937ec8f73cd8be2c74add5a911ba64df27458ed8229da804a26".as_bytes().try_into().unwrap();
//     let program_domain_name: String = String::from("http://localhost:8989/");
//     let program_redirect_uri: Vec<String> = vec![
//         "https://exzo.com/ru".to_string(),
//         "https://wallet.exzo.com/".to_string(),
//     ];

//     let (relying_party_address, bump_seed_nonce) = get_relying_party_address(
//         &program_name,
//         program_icon_cid,
//         &program_domain_name,
//         &program_redirect_uri,
//     );

//     let authority = Keypair::new();
//     let wrong_authority = Keypair::new();
//     let related_program = Keypair::new();
//     let recipient = Pubkey::new_unique();

//     let related_program_data: RelatedProgramInfo = RelatedProgramInfo {
//         name: program_name.clone(),
//         icon_cid: program_icon_cid,
//         domain_name: program_domain_name.clone(),
//         redirect_uri: program_redirect_uri.clone(),
//     };

//     let relying_party_data: RelyingPartyData = RelyingPartyData {
//         version: 1,
//         authority: authority.pubkey(),
//         related_program: related_program.pubkey(),
//         related_program_data: related_program_data,
//     };

//     initialize_relying_party_account(
//         &mut context,
//         &relying_party_address,
//         &authority,
//         &related_program.pubkey(),
//         &program_name,
//         program_icon_cid,
//         &program_domain_name,
//         &program_redirect_uri,
//         bump_seed_nonce,
//         get_instance_packed_len(&relying_party_data).unwrap(),
//     )
//     .await
//     .unwrap();

//     let transaction = Transaction::new_signed_with_payer(
//         &[Instruction::new_with_borsh(
//             id(),
//             &instruction::RelyingPartyInstruction::CloseAccount,
//             vec![
//                 AccountMeta::new(relying_party_address, false),
//                 AccountMeta::new_readonly(authority.pubkey(), false),
//                 AccountMeta::new(Pubkey::new_unique(), false),
//             ],
//         )],
//         Some(&context.payer.pubkey()),
//         &[&context.payer],
//         context.last_blockhash,
//     );
//     assert_eq!(
//         context
//             .banks_client
//             .process_transaction(transaction)
//             .await
//             .unwrap_err()
//             .unwrap(),
//         TransactionError::InstructionError(0, InstructionError::MissingRequiredSignature)
//     );
// }
