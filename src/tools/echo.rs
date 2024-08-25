use crate::args::EchoArgs;

pub fn echo_handler(args: EchoArgs) -> String {
    let EchoArgs {
        strings,
        omit_newline,
        enable_escape_characters,
        ..
    } = args;

    let mut result = vec![];

    if let Some(strings) = strings {
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
        result.push(combined_string);
    }

    if !omit_newline {
        result.push("\n".to_owned());
    }

    result.join("")
}
