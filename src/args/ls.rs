use clap::Args;

#[derive(Debug, Args)]
pub struct LsArgs {
    /// Shows detailed information about files (permissions, user, etc)
    #[arg(short = 'l')]
    pub long_list_format: bool,
    /// Shows all files (including the hidden ones)
    #[arg(short = 'a')]
    pub show_all_files: bool,
    /// Shows detailed information about files (permissions, user, etc)
    /// in human readable format
    #[arg(short = 'H')]
    pub human_readable_format: bool,
    /// Recursively list all the files
    #[arg(short = 'R')]
    pub list_recursively: bool,
    /// Show files sorted by last modified time
    #[arg(short = 't')]
    pub sort_dsc_by_modified_time: bool,
    /// Reverse the order of the output
    #[arg(short = 'r')]
    pub reversed_output: bool,
    /// Show files sorted by last modified time
    #[arg(short = 'S')]
    pub sort_dsc_by_file_size: bool,
    /// Show one file per line
    #[arg(short = '1')]
    pub show_one_file_per_line: bool,
}
