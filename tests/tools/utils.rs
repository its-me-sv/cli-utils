use cli_utils::args::EchoArgs;

pub fn echo_args_builder(
    strings: Option<Vec<String>>,
    disable_escape_characters: bool,
    enable_escape_characters: bool,
    omit_newline: bool,
) -> EchoArgs {
    EchoArgs {
        strings,
        disable_escape_characters,
        enable_escape_characters,
        omit_newline,
    }
}
