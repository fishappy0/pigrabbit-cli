use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "pigrabbit-cli",
    about = "A command line interface using PigRabbit API"
)]
pub struct Cli {
    pub name: Option<String>,

    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(
        name = "RetreiveRecord",
        about = "Retreiving record(s) information from PorkBun, with the given id or the subdomain and rtype"
    )]
    RetreiveRecord {
        #[clap(long, short)]
        domain: String,
        #[clap(long, short)]
        id: Option<String>,
        #[clap(long, short)]
        subdomain: Option<String>,
        #[clap(long, short)]
        rtype: Option<String>,
    },
    #[clap(
        name = "RetreiveSSL",
        about = "Retreiving SSL information from PorkBun, with the given domain"
    )]
    RetreiveSSL {
        #[clap(long, short)]
        domain: String,
    },
    #[clap(
        name = "DeleteRecord",
        about = "Deleting record(s) information from PorkBun, with the given id or the subdomain and rtype options"
    )]
    DeleteRecord(DeleteCommand),
    #[clap(
        name = "AddRecord",
        about = "Adding record(s) information to PorkBun, with the given content"
    )]
    AddRecord {
        #[clap(long, short)]
        domain: String,
        #[clap(long, short)]
        name: String,
        #[clap(long, short)]
        rtype: String,
        #[clap(long, short)]
        content: String,
        #[clap(long, short)]
        ttl: String,
    },
    #[clap(
        name = "UpdateRecord",
        about = "Updating record(s) information to PorkBun, with the given subdomain and rtype"
    )]
    EditRecord(EditCommand),
}
#[derive(clap::Args)]
pub struct EditCommand {
    #[clap(subcommand)]
    pub command: Option<EditOptions>,
}
#[derive(clap::Args)]
pub struct DeleteCommand {
    #[clap(subcommand)]
    pub command: Option<DeleteOptions>,
}
#[derive(Subcommand)]
pub enum DeleteOptions {
    #[clap(
        name = "ById",
        about = "Deleting record(s) information from PorkBun, with the given id"
    )]
    ById {
        #[clap(long, short)]
        domain: String,
        #[clap(long, short)]
        id: String,
    },
    #[clap(
        name = "BySubdomain",
        about = "Deleting record(s) information from PorkBun, with the given subdomain"
    )]
    BySubdomanAndType {
        #[clap(long, short)]
        domain: String,
        #[clap(long, short)]
        subdomain: String,
        #[clap(long, short)]
        rtype: String,
    },
}

#[derive(Subcommand)]
pub enum EditOptions {
    #[clap(
        name = "ById",
        about = "Updating record(s) information to PorkBun, with the given id"
    )]
    ById {
        #[clap(long, short)]
        domain: String,
        #[clap(long, short)]
        id: String,
        #[clap(long, short)]
        name: String,
        #[clap(long, short)]
        rtype: String,
        #[clap(long, short)]
        content: String,
        #[clap(long, short)]
        ttl: Option<String>,
    },
    #[clap(
        name = "BySubdomain",
        about = "Updating record(s) information to PorkBun, with the given subdomain"
    )]
    BySubdomanAndType {
        #[clap(long, short)]
        domain: String,
        #[clap(long, short)]
        subdomain: String,
        #[clap(long, short)]
        rtype: String,
        #[clap(long, short)]
        content: String,
        #[clap(long, short)]
        ttl: String,
    },
}
