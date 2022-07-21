use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "pigrabbit-cli",
    about = "A command line interface using PigRabbit API"
)]
pub struct Cli {
    /// Optional name to operate on
    #[clap(value_parser)]
    pub name: Option<String>,

    /// Sets a custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub(crate) debug: u8,

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
        #[clap(long, short, value_parser)]
        domain: String,
        #[clap(long, short, value_parser)]
        id: Option<String>,
        #[clap(long, short, value_parser)]
        subdomain: Option<String>,
        #[clap(long, short, value_parser)]
        rtype: Option<String>,
    },
    #[clap(
        name = "RetreiveSSL",
        about = "Retreiving SSL information from PorkBun, with the given domain"
    )]
    RetreiveSSL {
        #[clap(long, short, value_parser)]
        domain: String,
    },
    #[clap(
        name = "DeleteRecord",
        about = "Deleting record(s) information from PorkBun, with the given id or the subdomain and rtype options"
    )]
    DeleteRecord(DeleteComand),
    #[clap(
        name = "AddRecord",
        about = "Adding record(s) information to PorkBun, with the given subdomain and rtype"
    )]
    AddRecord {
        #[clap(long, short, value_parser)]
        domain: String,
        #[clap(long, short, value_parser)]
        name: String,
        #[clap(long, short, value_parser)]
        rtype: String,
        #[clap(long, short, value_parser)]
        content: String,
        #[clap(long, short, value_parser)]
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
pub struct DeleteComand {
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
        #[clap(long, short, value_parser)]
        domain: String,
        #[clap(long, short, value_parser)]
        id: String,
    },
    #[clap(
        name = "BySubdomain",
        about = "Deleting record(s) information from PorkBun, with the given subdomain"
    )]
    BySubdomanAndType {
        #[clap(long, short, value_parser)]
        domain: String,
        #[clap(long, short, value_parser)]
        subdomain: String,
        #[clap(long, short, value_parser)]
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
        #[clap(long, short, value_parser)]
        domain: String,
        #[clap(long, short, value_parser)]
        id: String,
        #[clap(long, short, value_parser)]
        name: String,
        #[clap(long, short, value_parser)]
        rtype: String,
        #[clap(long, short, value_parser)]
        content: String,
        #[clap(long, short, value_parser)]
        ttl: Option<String>,
    },
    #[clap(
        name = "BySubdomain",
        about = "Updating record(s) information to PorkBun, with the given subdomain"
    )]
    BySubdomanAndType {
        #[clap(long, short, value_parser)]
        domain: String,
        #[clap(long, short, value_parser)]
        subdomain: String,
        #[clap(long, short, value_parser)]
        rtype: String,
        #[clap(long, short, value_parser)]
        content: String,
        #[clap(long, short, value_parser)]
        ttl: String,
    },
}
