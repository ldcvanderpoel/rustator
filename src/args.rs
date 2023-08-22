// mod datatypes;

use clap::{Args, Parser, Subcommand};
// use datatypes::Order;

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct ConfigArgs {
    #[clap(subcommand)]
    pub command: CommandType,
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    /// Combines subdomains and keywords using separators.
    Combine(CombineCommand),
}

#[derive(Debug, Args)]
pub struct CombineCommand {
    /// File containing a list of subdomains
    pub subdomains: String,

    /// File containing a list of keywords.
    pub keywords: String,

    /// File containing a list of separators.
    pub separators: String,
}
