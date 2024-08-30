use clap::Args;

#[derive(Debug, Args)]
pub struct EchoArgs {
    /// The input to repeat
    pub strings: Option<Vec<String>>,
    /// Disables the interpretation of escape characters
    #[arg(short = 'E', default_value_t = true)]
    pub disable_escape_characters: bool,
    /// Enables the interpretation of escape characters
    #[arg(short = 'e')]
    pub enable_escape_characters: bool,
    /// Displays the output while omitting the newline after it
    #[arg(short = 'n')]
    pub omit_newline: bool,
}
