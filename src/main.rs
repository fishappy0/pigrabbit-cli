use clap::{ErrorKind, Parser};
use commands::Cli;
use directories::BaseDirs;
use std::fs::File;
use std::path::Path;
use tokio;
mod commands;

/// Generate a keypair and save it to the config folder.
fn create_config_file(json_config_dir: &Path) -> pigrabbit::types::Keys {
    let mut apikey = "".to_owned();
    let mut secretapikey = "".to_owned();
    println!("[PigRabbit] The API and secret API Keys not found!");
    println!("[PigRabbit] Please enter the API key: ");
    std::io::stdin().read_line(&mut apikey).unwrap();
    println!("[PigRabbit] Please enter the secret API key: ");
    std::io::stdin().read_line(&mut secretapikey).unwrap();

    let key_struct = pigrabbit::types::Keys {
        apikey: apikey.trim().to_string(),
        secretapikey: secretapikey.trim().to_string(),
    };
    serde_json::to_writer(File::create(&json_config_dir).unwrap(), &key_struct).unwrap();
    key_struct
}

/// Checks if the config folder exists. If not, it creates it.
fn read_existing_dir_or_create(config_dir: &Path) {
    match std::fs::read_dir(config_dir) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            std::fs::create_dir(config_dir).unwrap()
        }
        Err(e) => panic!("Error: {}", e),
        _ => (),
    }
}

/// Grab the keypair from the config file or generate a new one if it doesn't exist.
fn get_keys(config_dir: &Path, config_file: &Path) -> pigrabbit::types::Keys {
    read_existing_dir_or_create(&config_dir);
    match std::fs::read_to_string(config_file) {
        Ok(v) => serde_json::from_str(&v).expect("Failed to parse config file"),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => create_config_file(config_file),
        // Panic if there is an error other than file not found.
        Err(e) => panic!("{}", e),
    }
}
async fn generate_client_with_config_file(
    config_file: Option<&std::path::PathBuf>,
) -> pigrabbit::PRClient {
    let key_struct;
    match config_file {
        Some(v) => {
            key_struct = get_keys(v.parent().unwrap(), v);
        }
        None => {
            let base_dirs = BaseDirs::new().unwrap();
            let config_dir = base_dirs.config_dir().join("pigrabbit-cli");
            let json_config_dir = config_dir.join("config.json");
            key_struct = get_keys(&config_dir, &json_config_dir);
        }
    }

    pigrabbit::PRClient::new(key_struct)
}

async fn execute_command(cli_instance: &Cli, config_file: Option<&std::path::PathBuf>) {
    let mut prclient = generate_client_with_config_file(config_file).await;
    match &cli_instance.command {
        // Retreiving record by providing either id or subdomain and record_type.
        commands::Commands::RetreiveRecord {
            id,
            subdomain,
            record_type,
        } => {
            if id.to_owned() != None && record_type.to_owned() != None {
                panic!("[PigRabbit] {}", ErrorKind::ArgumentConflict);
            }

            let subdomain_result = match record_type {
                Some(record_type) => {
                    let subdomain_name = match subdomain {
                        Some(subdomain) => subdomain,
                        None => "",
                    };

                    let res = prclient
                        .retreive_by_type_with_subdomain(
                            &cli_instance.domain,
                            &record_type,
                            &subdomain_name,
                        )
                        .await;
                    println!("{}", serde_yaml::to_string(&res.unwrap()).unwrap());
                    true
                }
                None => false,
            };
            let id_result = match id {
                Some(rid) => {
                    let res = prclient
                        .retreive_by_domain_with_id(&cli_instance.domain, &rid)
                        .await;
                    println!("{}", serde_yaml::to_string(&res.unwrap()).unwrap());
                    true
                }
                None => false,
            };

            if id_result == false && subdomain_result == false {
                let res = prclient
                    .retreive_by_domain_with_id(&cli_instance.domain, "")
                    .await;
                println!("{}", serde_yaml::to_string(&res.unwrap()).unwrap());
            }
        }
        // Retreives the ssl certificate for a domain.
        commands::Commands::RetreiveSSL {} => {
            let res = prclient.retreive_ssl_by_domain(&cli_instance.domain).await;
            println!("{}", serde_yaml::to_string(&res.unwrap()).unwrap());
        }
        // Deletes a record by each options.
        commands::Commands::DeleteRecord(delete_by) => match &delete_by.command {
            // Delete a record by id.
            commands::DeleteOptions::ById { id } => {
                prclient.del_by_id(&cli_instance.domain, &id).await.unwrap();
                println!("[PigRabbit] Deleted successfully!");
            }
            // Delete a record by subdomain and record type.
            commands::DeleteOptions::BySubdomainAndType {
                subdomain,
                record_type,
            } => {
                prclient
                    .del_by_type_with_subdomain(&record_type, &cli_instance.domain, &subdomain)
                    .await
                    .unwrap();
                println!("[PigRabbit] Deleted successfully!");
            }
        },
        // Creates a new record.
        commands::Commands::AddRecord {
            name,
            record_type,
            content,
            ttl,
        } => {
            let record = pigrabbit::types::Record {
                name: name.to_owned(),
                dtype: record_type.to_owned(),
                content: content.to_owned(),
                ttl: ttl.to_string(),
            };
            let res = prclient.add_record(&cli_instance.domain, &record).await;
            println!("{}", serde_yaml::to_string(&res.unwrap()).unwrap());
        }
        // Updates a record by each options.
        commands::Commands::EditRecord(edit_by) => match &edit_by.command {
            // Update a record by id.
            commands::EditOptions::ById {
                id,
                name,
                record_type,
                content,
                ttl,
            } => {
                let record = pigrabbit::types::Record {
                    name: name.to_owned(),
                    dtype: record_type.to_owned(),
                    content: content.to_owned(),
                    ttl: ttl.to_owned().unwrap_or("".to_owned()),
                };
                let res = prclient
                    .edit_by_domain_and_id(&cli_instance.domain, &id, &record)
                    .await;
                println!("{}", serde_yaml::to_string(&res.unwrap()).unwrap());
            }
            // Update a record by subdomain and record type.
            commands::EditOptions::BySubdomainAndType {
                subdomain,
                record_type,
                content,
                ttl,
            } => {
                let record_type = &record_type;
                let record = pigrabbit::types::Record {
                    name: "".to_owned(),
                    dtype: record_type.to_string(),
                    content: content.to_owned(),
                    ttl: ttl.to_owned(),
                };
                let res = prclient
                    .edit_by_domain_subdomain_and_type(&cli_instance.domain, &subdomain, &record)
                    .await;
                println!("{}", serde_yaml::to_string(&res.unwrap()).unwrap());
            }
        },
    }
}
#[tokio::main]
async fn main() {
    let cli = commands::Cli::parse();
    //Run command
    execute_command(&cli, cli.config.as_ref()).await;
}
