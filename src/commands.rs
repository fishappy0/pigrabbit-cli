use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "pigrabbit-cli",
    about = "A command line interface using PigRabbit API"
)]
pub struct Cli {
    // Set the configuration file to use.
    #[clap(short, long)]
    pub config: Option<std::path::PathBuf>,

    #[clap(subcommand)]
    pub command: Commands,

    pub domain: String,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(
        about = "Retreiving record(s) information from PorkBun, with the given id or the subdomain and record type"
    )]
    RetreiveRecord {
        #[clap(long, short)]
        id: Option<String>,
        #[clap(long, short)]
        subdomain: Option<String>,
        #[clap(long = "record-type", short = 't')]
        record_type: Option<String>,
    },
    #[clap(about = "Retreiving SSL information from PorkBun, with the given domain")]
    RetreiveSSL {},
    #[clap(
        about = "Deleting record(s) information from PorkBun, with the given id or the subdomain and record type options"
    )]
    DeleteRecord(DeleteCommand),
    #[clap(about = "Adding record(s) information to PorkBun, with the given content")]
    AddRecord {
        #[clap(long, short)]
        name: String,
        #[clap(long = "record-type", short = 't')]
        record_type: String,
        #[clap(long, short)]
        content: String,
        #[clap(long, default_value = "300")]
        ttl: i32,
    },
    #[clap(
        about = "Updating record(s) information to PorkBun, with the given subdomain and record type"
    )]
    EditRecord(EditCommand),
}
#[derive(clap::Args)]
pub struct EditCommand {
    #[clap(subcommand)]
    pub command: EditOptions,
}
#[derive(clap::Args)]
pub struct DeleteCommand {
    #[clap(subcommand)]
    pub command: DeleteOptions,
}
#[derive(Subcommand)]
pub enum DeleteOptions {
    #[clap(about = "Deleting record(s) information from PorkBun, with the given id")]
    ById {
        #[clap(long, short)]
        id: String,
    },
    #[clap(about = "Deleting record(s) information from PorkBun, with the given subdomain")]
    BySubdomainAndType {
        #[clap(long, short)]
        subdomain: String,
        #[clap(long = "record-type", short = 't')]
        record_type: String,
    },
}

#[derive(Subcommand)]
pub enum EditOptions {
    #[clap(about = "Updating record(s) information to PorkBun, with the given id")]
    ById {
        #[clap(long, short)]
        id: String,
        #[clap(long, short)]
        name: String,
        #[clap(long = "record-type", short = 't')]
        record_type: String,
        #[clap(long, short)]
        content: String,
        #[clap(long)]
        ttl: Option<String>,
    },
    #[clap(about = "Updating record(s) information to PorkBun, with the given subdomain")]
    BySubdomainAndType {
        #[clap(long, short)]
        subdomain: String,
        #[clap(long = "record-type", short = 't')]
        record_type: String,
        #[clap(long, short)]
        content: String,
        #[clap(long)]
        ttl: String,
    },
}
