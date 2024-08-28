use crate::args::EchoArgs;

pub fn echo_handler(args: EchoArgs) -> String {
    let EchoArgs {
        strings,
        omit_newline,
        enable_escape_characters,
        ..
    } = args;

    // No input was provided
    if strings.is_none() {
        return String::from(if !omit_newline { "\n" } else { "" });
    }

    let strings = strings.unwrap();

    let mut combined_string = strings
        .iter()
        .map(|string| string.trim().to_owned())
        .collect::<Vec<String>>()
        .join(" ");

    if enable_escape_characters {
        combined_string = combined_string
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t")
            .replace("\\'", "\'")
            .replace("\\\"", "\"")
            .replace("\\\\", "\\");
    }

    if !omit_newline {
        return format!("{combined_string}\n");
    }

    combined_string
}
