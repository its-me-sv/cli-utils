use crate::args::EchoArgs;

pub struct EchoParser {
    args: EchoArgs,
}

impl EchoParser {
    pub fn new(args: EchoArgs) -> Self {
        Self { args }
    }

    pub fn parse(&self) -> String {
        self.args
            .strings
            .as_ref()
            .map_or(String::from("\n"), |strings| {
                self.handle_input_strings(strings)
            })
    }

    fn handle_input_strings(&self, strings: &[String]) -> String {
        let estimated_size = strings.iter().map(|string| string.len()).sum();
        let mut combined_strings = String::with_capacity(estimated_size);

        strings.iter().for_each(|string| {
            combined_strings.push_str(string.trim());
            combined_strings.push(' ');
        });

        combined_strings.pop();

        if self.args.enable_escape_characters && combined_strings.contains('\\') {
            [
                ("\\n", "\n"),
                ("\\r", "\r"),
                ("\\t", "\t"),
                ("\\'", "\'"),
                ("\\\"", "\""),
                ("\\\\", "\\"),
            ]
            .into_iter()
            .for_each(|(from, to)| {
                if combined_strings.contains(from) {
                    combined_strings = combined_strings.replace(from, to);
                }
            });
        }

        if !self.args.omit_newline {
            combined_strings.push('\n');
        }

        combined_strings
    }
}
