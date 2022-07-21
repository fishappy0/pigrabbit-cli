use clap::Parser;
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

#[tokio::main]
async fn main() {
    let base_dirs = BaseDirs::new().unwrap();
    let config_dir = base_dirs.config_dir().join("pigrabbit-cli");
    let json_config_dir = config_dir.join("config.json");

    let key_struct = get_keys(&config_dir, &json_config_dir);
    let mut prclient = pigrabbit::PRClient::new(key_struct);

    let cli = commands::Cli::parse();
    match cli.command {
        // Retreiving record by providing either id or subdomain and rtype.
        Some(commands::Commands::RetreiveRecord {
            domain,
            id,
            subdomain,
            rtype,
        }) => {
            print!(
                "id: {:?}, subdomain: {:?}, rtype: {:?}",
                id, subdomain, rtype
            );
            if id != None && rtype != None {
                panic!("[PigRabbit] You can only either retreive a record by id or by subdomain and record type");
            }

            let subdomain_result = match rtype {
                Some(record_type) => {
                    let subdomain_name = match subdomain {
                        Some(subdomain) => subdomain,
                        None => "".to_owned(),
                    };

                    let res = prclient
                        .retreive_by_type_with_subdomain(&domain, &record_type, &subdomain_name)
                        .await;
                    println!("{:#?}", res);
                    true
                }
                None => false,
            };
            let id_result = match id {
                Some(rid) => {
                    let res = prclient.retreive_by_domain_with_id(&domain, &rid).await;
                    println!("{:#?}", res);
                    true
                }
                None => false,
            };

            if id_result == false && subdomain_result == false {
                let res = prclient.retreive_by_domain_with_id(&domain, "").await;
                println!("{:#?}", res);
            }
        }
        // Retreives the ssl certificate for a domain.
        Some(commands::Commands::RetreiveSSL { domain }) => {
            let res = prclient.retreive_ssl_by_domain(&domain).await;
            println!("{:#?}", res);
        }
        // Deletes a record by each options.
        Some(commands::Commands::DeleteRecord(delete_by)) => match delete_by.command {
            // Delete a record by id.
            Some(commands::DeleteOptions::ById { domain, id }) => {
                let res = prclient.del_by_id(&domain, &id).await;
                println!("{:#?}", res);
            }
            // Delete a record by subdomain and record type.
            Some(commands::DeleteOptions::BySubdomanAndType {
                domain,
                subdomain,
                rtype,
            }) => {
                let res = prclient
                    .del_by_type_with_subdomain(&rtype, &domain, &subdomain)
                    .await;
                println!("{:#?}", res);
            }
            _ => println!("[PigRabbit] Invalid arguments"),
        },
        // Creates a new record.
        Some(commands::Commands::AddRecord {
            domain,
            name,
            rtype,
            content,
            ttl,
        }) => {
            let record = pigrabbit::types::Record {
                name: name,
                dtype: rtype,
                content: content,
                ttl: ttl,
            };
            let res = prclient.add_record(&domain, &record).await;
            println!("{:#?}", res);
        }
        // Updates a record by each options.
        Some(commands::Commands::EditRecord(edit_by)) => match edit_by.command {
            // Update a record by id.
            Some(commands::EditOptions::ById {
                domain,
                id,
                name,
                rtype,
                content,
                ttl,
            }) => {
                let record = pigrabbit::types::Record {
                    name: name,
                    dtype: rtype,
                    content: content,
                    ttl: ttl.unwrap_or("".to_owned()),
                };
                let res = prclient.edit_by_domain_and_id(&domain, &id, &record).await;
                println!("{:#?}", res);
            }
            // Update a record by subdomain and record type.
            Some(commands::EditOptions::BySubdomanAndType {
                domain,
                subdomain,
                rtype,
                content,
                ttl,
            }) => {
                let record_type = &rtype;
                let record = pigrabbit::types::Record {
                    name: "".to_owned(),
                    dtype: rtype.to_owned(),
                    content: content,
                    ttl: ttl,
                };
                let res = prclient
                    .edit_by_domain_subdomain_and_type(&domain, &subdomain, record_type, &record)
                    .await;
                println!("{:#?}", res);
            }
            _ => println!("[PigRabbit] Invalid arguments"),
        },
        None => println!("[PigRabbit] No command specified"),
    }
}
