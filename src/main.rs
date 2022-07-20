use directories::BaseDirs;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use tokio;

// create a new folder in the config directory
fn create_config_folder(config_dir: &Path) {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&config_dir)
        .unwrap();
}
// Generate a keypair and save it to the config folder.
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
// Grab the keypair from the config file or generate a new one if it doesn't exist.
fn get_keys(config_dir: &Path, config_file: &Path) -> pigrabbit::types::Keys {
    read_existing_dir_or_create(&config_dir);
    match std::fs::read_to_string(config_file) {
        Ok(v) => serde_json::from_str(&v).expect("Failed to parse config file"),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => create_config_file(config_file),
        // Panic if there is an error other than file not found.
        Err(e) => panic!("{}", e),
    }
}

fn read_existing_dir_or_create(config_dir: &Path) {
    std::fs::read_dir(config_dir)
        .map_err(|e| match e {
            e if e.kind() == std::io::ErrorKind::NotFound => create_config_folder(config_dir),
            e => println!("Error: {}", e),
        })
        .unwrap();
}

#[tokio::main]
async fn main() {
    let base_dirs = BaseDirs::new().unwrap();
    let config_dir = base_dirs.config_dir().join("pigrabbit-cli");
    let json_config_dir = config_dir.join("config.json");

    let key_struct = get_keys(&config_dir, &json_config_dir);
    let mut prclient = pigrabbit::PRClient::new(key_struct);

    let result = prclient
        .retreive_ssl_by_domain(
            "arvinderd.com",
            // "", // none,
        )
        .await
        .unwrap();

    // print result
    println!("{:#?}", result);
}
