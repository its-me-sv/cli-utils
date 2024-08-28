use clap::Args;

#[derive(Debug, Args)]
pub struct CatArgs {
    /// Name of the file(s)
    pub file_names: Option<Vec<String>>,
    /// Shows all characters
    #[arg(short = 'A')]
    pub show_all_characters: bool,
    /// Creates a numbered list with non-blank lines
    #[arg(short = 'b')]
    pub numbered_list_excluding_non_blank_lines: bool,
    /// Shows non-printing characters and ends lines with $
    #[arg(short = 'e')]
    pub show_non_printing_characters_and_end_with_dollar: bool,
    /// Displays a $ at the end of each line
    #[arg(short = 'E')]
    pub display_dollar_at_line_end: bool,
    /// Creates a numbered list with all lines, including blank lines
    #[arg(short = 'n')]
    pub numbered_list_including_blank_lines: bool,
    /// Squeeze multiple adjacent blank lines into a single blank line
    #[arg(short = 's')]
    pub squeeze_adjacent_blank_lines: bool,
    /// Shows tab characters as ^I
    #[arg(short = 'T')]
    pub show_tab_as_i: bool,
    /// Displays non-printing characters, except for tabs and end-of-line characters
    #[arg(short = 'v')]
    pub show_non_printing_characters_except_tabs_and_eol: bool,
}
