mod cat;
mod echo;
mod find;
mod grep;
mod ls;

pub use cat::CatArgs;
use clap::{Parser, Subcommand};
pub use echo::EchoArgs;
use ls::LsArgs;

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
    Cat(CatArgs),
    /// Lists directories
    Ls(LsArgs),
    /// Locates files or directories
    Find,
    /// Matches text in files
    Grep,
}
