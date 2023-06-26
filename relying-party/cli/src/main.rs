use {
    borsh::{BorshDeserialize, BorshSerialize},
    cid::Cid,
    clap::{
        crate_description, crate_name, crate_version, value_t, value_t_or_exit, App, AppSettings,
        Arg, SubCommand,
    },
    solana_clap_utils::{
        input_parsers::pubkey_of,
        input_validators::{is_amount, is_keypair, is_url, is_valid_pubkey},
    },
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        hash::Hasher,
        native_token::lamports_to_sol,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
        system_instruction,
        transaction::Transaction,
    },
    std::cmp::min,
    std::convert::TryFrom,
    vpl_relying_party::{
        id,
        state::{RelatedProgramInfo, RelyingPartyData},
    },
};

struct Config {
    fee_payer: Keypair,
    authority: Keypair,
    json_rpc_url: String,
    #[allow(dead_code)] // TODO: Add in new version
    verbose: bool,
    dry_run: bool,
}

fn send_transaction(
    rpc_client: &RpcClient,
    config: &Config,
    transaction: Transaction,
) -> solana_client::client_error::Result<()> {
    if config.dry_run {
        let result = rpc_client.simulate_transaction(&transaction)?;
        println!("Simulate result: {:?}", result);
    } else {
        let signature = rpc_client.send_and_confirm_transaction_with_spinner(&transaction)?;
        println!("Signature: {}", signature);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg({
            let arg = Arg::with_name("config_file")
                .short("C")
                .long("config")
                .value_name("PATH")
                .takes_value(true)
                .global(true)
                .help("Configuration file to use");
            if let Some(ref config_file) = *solana_cli_config::CONFIG_FILE {
                arg.default_value(config_file)
            } else {
                arg
            }
        })
        .arg(
            Arg::with_name("fee_payer")
                .long("fee-payer")
                .value_name("KEYPAIR")
                .validator(is_keypair)
                .takes_value(true)
                .global(true)
                .help("Filepath or URL to a fee-payer keypair [default: client keypair]"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .takes_value(false)
                .global(true)
                .help("Show additional information"),
        )
        .arg(
            Arg::with_name("json_rpc_url")
                .long("url")
                .value_name("URL")
                .takes_value(true)
                .global(true)
                .validator(is_url)
                .help("JSON RPC URL for the cluster [default: value from configuration file]"),
        )
        .arg(
            Arg::with_name("authority")
                .long("authority")
                .value_name("KEYPAIR")
                .validator(is_keypair)
                .takes_value(true)
                .global(true)
                .help(
                    "Filepath or URL to current relying-party authority keypair. [default: client keypair]",
                ),
        )
        .subcommand(
            SubCommand::with_name("account")
                .about("Display information stored in relying-party account")
                .arg(
                    Arg::with_name("relying_party")
                        .value_name("RELYING_PARTY_ADDRESS")
                        .validator(is_valid_pubkey)
                        .index(1)
                        .required(true)
                        .help("Address of the relying-party account"),
                ),
        )
        .subcommand(
            SubCommand::with_name("create-account")
                .about("Create a new relying-party account")
                .arg(
                    Arg::with_name("program_name")
                        .long("program-name")
                        .value_name("PROGRAM_NAME")
                        .validator(is_valid_program_name)
                        .index(1)
                        .required(true)
                        .help("The display name associated with relying-party account"),
                )
                .arg(
                    Arg::with_name("program_icon_cid")
                        .long("program-icon-cid")
                        .value_name("ICON_CID")
                        .validator(is_valid_cid)
                        .index(2)
                        .required(true)
                        .help(
                            "Content identifier of the icon associated with relying-party account: https://docs.ipfs.io/concepts/content-addressing/",
                        ),
                )
                .arg(
                    Arg::with_name("program_domain_name")
                        .long("program-domain-name")
                        .value_name("URL")
                        .validator(is_url)
                        .index(3)
                        .required(true)
                        .help("Domain name associated with relying-party account"),
                )
                .arg(
                    Arg::with_name("program_redirect_uris")
                        .long("program-redirect-uris")
                        .value_name("REDIRECT_URIs")
                        .validator(is_url)
                        .index(4)
                        .required(true)
                        .multiple(true)
                        .min_values(1)
                        .help("Allowed URIs for end-user to be redirected to"),
                )
                .arg(
                    Arg::with_name("lamports")
                        .long("lamports")
                        .value_name("AMOUNT")
                        .validator(is_amount)
                        .help("Lamports to create a new relying-party account"),
                ),
        )
        .subcommand(
            SubCommand::with_name("set-authority")
                .about("Set close authority of the relying-party account")
                .arg(
                    Arg::with_name("relying_party")
                        .value_name("RELYING_PARTY_ADDRESS")
                        .validator(is_valid_pubkey)
                        .index(1)
                        .required(true)
                        .help("The address of the relying-party account"),
                )
                .arg(
                    Arg::with_name("new_authority")
                        .value_name("NEW_AUTHORITY_ADDRESS")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .help("New authority of the relying-party account"),
                ),
        )
        .subcommand(
            SubCommand::with_name("close-account")
                .about("Close relying-party account")
                .arg(
                    Arg::with_name("relying_party")
                        .value_name("RELYING_PARTY_ADDRESS")
                        .validator(is_valid_pubkey)
                        .index(1)
                        .required(true)
                        .help("The address of the relying-party account"),
                )
                .arg(
                    Arg::with_name("receiver")
                        .value_name("RECEIVER_ADDRESS")
                        .validator(is_valid_pubkey)
                        .required(true)
                        .help("The address to send lamports from relying-party"),
                ),
        )
        .get_matches();

    let (sub_command, sub_matches) = app_matches.subcommand();
    let matches = sub_matches.unwrap();

    let config = {
        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            solana_cli_config::Config::load(config_file).unwrap_or_default()
        } else {
            solana_cli_config::Config::default()
        };

        Config {
            json_rpc_url: matches
                .value_of("json_rpc_url")
                .unwrap_or(&cli_config.json_rpc_url)
                .to_string(),
            fee_payer: read_keypair_file(
                matches
                    .value_of("fee_payer")
                    .unwrap_or(&cli_config.keypair_path),
            )?,
            authority: read_keypair_file(
                matches
                    .value_of("authority")
                    .unwrap_or(&cli_config.keypair_path),
            )?,
            verbose: matches.is_present("verbose"),
            dry_run: matches.is_present("dry_run"),
        }
    };
    solana_logger::setup_with_default("solana=info");
    let rpc_client =
        RpcClient::new_with_commitment(config.json_rpc_url.clone(), CommitmentConfig::confirmed());

    match (sub_command, sub_matches) {
        ("account", Some(arg_matches)) => {
            let relying_party_address = pubkey_of(arg_matches, "relying_party").unwrap();
            let relying_party_data =
                get_relying_party_account(&rpc_client, &relying_party_address)?;
            let icon_cid = Cid::try_from(relying_party_data.related_program_data.icon_cid)?;

            println!("\nPublic Key: {}", relying_party_address);
            println!("-------------------------------------------------------------------");

            println!(
                "Relying Party Data:\n    \
                    version: {}\n    \
                    authority: {}\n    \
                    Releted program data:\n        \
                    name: {}\n        \
                    icon_cid: {}\n        \
                    domain_name: {}\n        \
                    redirect_uri: {:?}\n",
                relying_party_data.version,
                relying_party_data.authority,
                relying_party_data.related_program_data.name,
                icon_cid.to_string(),
                relying_party_data.related_program_data.domain_name,
                relying_party_data.related_program_data.redirect_uri,
            );

            Ok(())
        }
        ("create-account", Some(arg_matches)) => {
            let program_name = value_t_or_exit!(arg_matches, "program_name", String);
            let program_icon_cid = value_t_or_exit!(arg_matches, "program_icon_cid", String);
            let program_domain_name = value_t_or_exit!(arg_matches, "program_domain_name", String);
            let program_redirect_uris: Vec<_> = arg_matches
                .values_of("program_redirect_uris")
                .unwrap()
                .map(|s| s.to_string())
                .collect();
            let lamports = value_t!(arg_matches, "lamports", u64);

            process_initialize(
                &rpc_client,
                &config,
                &config.authority.pubkey(),
                &program_name,
                &program_icon_cid,
                &program_domain_name,
                program_redirect_uris.as_slice(),
                lamports.ok(),
            )
        }
        ("set-authority", Some(arg_matches)) => {
            let relying_party_address = pubkey_of(arg_matches, "relying_party").unwrap();
            let new_authority = pubkey_of(arg_matches, "new_authority").unwrap();

            process_set_authority(&rpc_client, &config, &relying_party_address, &new_authority)
        }
        ("close-account", Some(arg_matches)) => {
            let relying_party_address = pubkey_of(arg_matches, "relying_party").unwrap();
            let receiver = pubkey_of(arg_matches, "receiver").unwrap();

            process_close_account(&rpc_client, &config, &relying_party_address, &receiver)
        }
        _ => unreachable!(),
    }
}

fn get_relying_party_account(
    rpc_client: &RpcClient,
    relying_party_address: &Pubkey,
) -> Result<RelyingPartyData, String> {
    let account = rpc_client
        .get_multiple_accounts(&[*relying_party_address])
        .map_err(|err| err.to_string())?
        .into_iter()
        .next()
        .unwrap();

    match account {
        None => Err(format!(
            "Relying party {} does not exist",
            relying_party_address
        )),
        Some(account) => RelyingPartyData::try_from_slice(&account.data).map_err(|err| {
            format!(
                "Failed to deserialize feature proposal {}: {}",
                relying_party_address, err
            )
        }),
    }
}

fn get_relying_party_address(
    program_name: &str,
    program_icon_cid: &str,
    program_domain_name: &str,
    program_redirect_uris: &[String],
) -> (Pubkey, u8) {
    let mut hasher = Hasher::default();
    for uri in program_redirect_uris.iter() {
        hasher.hash(uri.as_bytes());
    }
    let program_redirect_uriss_hash = hasher.result();

    Pubkey::find_program_address(
        &[
            &program_name.as_bytes()[..min(32, program_name.len())],
            &program_icon_cid.as_bytes()[..min(32, program_icon_cid.len())],
            &program_domain_name.as_bytes()[..min(32, program_domain_name.len())],
            &program_redirect_uriss_hash.to_bytes()[..32],
        ],
        &id(),
    )
}

fn get_min_required_lamports(
    rpc_client: &RpcClient,
    authority_address: &Pubkey,
    program_name: &str,
    program_icon_cid: Vec<u8>,
    program_domain_name: &str,
    program_redirect_uris: Vec<String>,
) -> Result<u64, String> {
    let relying_party_data = RelyingPartyData {
        version: 1,
        authority: *authority_address,
        related_program_data: RelatedProgramInfo {
            name: program_name.to_string(),
            domain_name: program_domain_name.to_string(),
            icon_cid: program_icon_cid,
            redirect_uri: program_redirect_uris,
        },
    };

    let relying_party_data_size = relying_party_data.try_to_vec().unwrap().len();
    rpc_client
        .get_minimum_balance_for_rent_exemption(relying_party_data_size)
        .map_err(|err| err.to_string())
}

#[allow(clippy::too_many_arguments)]
fn process_initialize(
    rpc_client: &RpcClient,
    config: &Config,
    authority_address: &Pubkey,
    program_name: &str,
    program_icon_cid: &str,
    program_domain_name: &str,
    program_redirect_uris: &[String],
    lamports: Option<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let program_icon_cid_decoded = Cid::try_from(program_icon_cid);
    if program_icon_cid_decoded.is_err() {
        return Err(format!("Unable to parse icon cid: {}", &program_icon_cid).into());
    }

    let min_required_lamports = get_min_required_lamports(
        rpc_client,
        authority_address,
        program_name,
        program_icon_cid_decoded.unwrap().to_bytes(),
        program_domain_name,
        program_redirect_uris.to_owned(),
    )?;

    let lamports = if let Some(lamports) = lamports {
        if lamports < min_required_lamports {
            return Err(format!("Need at least {} lamports", min_required_lamports).into());
        }

        lamports
    } else {
        min_required_lamports
    };

    let balance = rpc_client.get_balance(&config.fee_payer.pubkey())?;
    if balance < lamports {
        return Err(format!(
            "Fee payer, {}, has insufficient balance: {} SOL required, {} SOL available",
            config.fee_payer.pubkey(),
            lamports_to_sol(lamports),
            lamports_to_sol(balance)
        )
        .into());
    }

    let (relying_party_address, bump_seed_nonce) = get_relying_party_address(
        program_name,
        program_icon_cid,
        program_domain_name,
        program_redirect_uris,
    );

    if get_relying_party_account(rpc_client, &relying_party_address).is_ok() {
        return Err(format!(
            "Relying Party Account {} already exists",
            &relying_party_address,
        )
        .into());
    }

    let mut initialize_transaction = Transaction::new_with_payer(
        &[
            system_instruction::transfer(
                &config.fee_payer.pubkey(),
                &relying_party_address,
                lamports,
            ),
            vpl_relying_party::instruction::initialize(
                &relying_party_address,
                authority_address,
                program_name.to_string(),
                program_icon_cid.to_string(),
                program_domain_name.to_string(),
                program_redirect_uris.to_vec(),
                bump_seed_nonce,
            ),
        ],
        Some(&config.fee_payer.pubkey()),
    );

    let blockhash = rpc_client.get_recent_blockhash()?.0;
    initialize_transaction.try_sign(&[&config.fee_payer], blockhash)?;

    send_transaction(rpc_client, config, initialize_transaction)?;

    println!("Relying Party Address: {}", relying_party_address);
    println!("Created!");
    Ok(())
}

fn process_set_authority(
    rpc_client: &RpcClient,
    config: &Config,
    relying_party_address: &Pubkey,
    new_authority_address: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let relying_party_account = get_relying_party_account(rpc_client, relying_party_address)?;

    if config.authority.pubkey() != relying_party_account.authority {
        return Err(format!(
            "Authority mistmach in Relying Party Account: {}\n \
            Provided authority: {}\n \
            Relying Party authority: {}",
            &relying_party_address,
            &config.authority.pubkey(),
            relying_party_account.authority
        )
        .into());
    }

    let mut set_authority_transaction = Transaction::new_with_payer(
        &[vpl_relying_party::instruction::set_authority(
            relying_party_address,
            &config.authority.pubkey(),
            new_authority_address,
        )],
        Some(&config.fee_payer.pubkey()),
    );

    let blockhash = rpc_client.get_recent_blockhash()?.0;
    set_authority_transaction.try_sign(&[&config.fee_payer, &config.authority], blockhash)?;

    send_transaction(rpc_client, config, set_authority_transaction)?;

    println!(
        "Authority changed from {} to {} successful!",
        config.authority.pubkey(),
        new_authority_address
    );
    Ok(())
}

fn process_close_account(
    rpc_client: &RpcClient,
    config: &Config,
    relying_party_address: &Pubkey,
    receiver: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let relying_party_account = get_relying_party_account(rpc_client, relying_party_address)?;

    if config.authority.pubkey() != relying_party_account.authority {
        return Err(format!(
            "Authority mistmach in Relying Party Account: {}, \
            provided authority: {}, \
            relying-party authority: {}",
            &relying_party_address,
            &config.authority.pubkey(),
            relying_party_account.authority
        )
        .into());
    }

    let mut close_account_transaction = Transaction::new_with_payer(
        &[vpl_relying_party::instruction::close_account(
            relying_party_address,
            &config.authority.pubkey(),
            receiver,
        )],
        Some(&config.fee_payer.pubkey()),
    );

    let blockhash = rpc_client.get_recent_blockhash()?.0;
    close_account_transaction.try_sign(&[&config.fee_payer, &config.authority], blockhash)?;

    send_transaction(rpc_client, config, close_account_transaction)?;

    println!("Relying-party account closed successful!");
    Ok(())
}

pub fn is_valid_cid(cid: String) -> Result<(), String> {
    let icon_cid = Cid::try_from(cid.clone());
    if icon_cid.is_ok() {
        Ok(())
    } else {
        Err(format!(
            "Unable to parse icon cid in base58 format: {}",
            &cid
        ))
    }
}

pub fn is_valid_program_name(name: String) -> Result<(), String> {
    let is_valid_name = name.chars().any(|c| c.is_ascii());
    if is_valid_name {
        Ok(())
    } else {
        Err(format!("Invalid `program-name`: {}", &name))
    }
}
