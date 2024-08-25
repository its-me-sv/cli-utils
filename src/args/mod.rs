mod cat;
mod echo;
mod find;
mod grep;
mod ls;

use clap::{Parser, Subcommand};
pub use echo::EchoArgs;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliUtilsArgs {
    #[command(subcommand)]
    pub tool: Tool,
}

#[derive(Debug, Subcommand)]
pub enum Tool {
    /// Repeats input
    Echo(EchoArgs),
    /// Concatenates files
    Cat,
    /// Lists directories
    Ls,
    /// Locates files or directories
    Find,
    /// Matches text in files
    Grep,
}
