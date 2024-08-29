use std::fs;

use crate::args::CatArgs;

pub struct CatParser {
    args: CatArgs,
}

impl CatParser {
    pub fn new(args: CatArgs) -> Self {
        Self { args }
    }

    pub fn parse(&self) -> String {
        self.args
            .file_names
            .as_ref()
            .map_or(String::with_capacity(0), |file_names| {
                self.handle_file_names(file_names)
            })
    }

    fn handle_file_names(&self, file_names: &[String]) -> String {
        let estimated_size = file_names
            .iter()
            .map(|name| fs::metadata(name).map(|m| m.len() as usize).unwrap_or(0))
            .sum();
        let mut result = String::with_capacity(estimated_size);
        let mut line_count = 1;

        file_names
            .iter()
            .for_each(|file_name| match fs::read_to_string(file_name) {
                Ok(file_content) => {
                    result.push_str(&self.parse_file_contents(file_content, &mut line_count))
                }
                Err(_) => {
                    result.push_str(&format!("cat: {}: No such file or directory\n", file_name))
                }
            });

        result
    }

    fn parse_file_contents(&self, file_content: String, line_count: &mut u128) -> String {
        let total_lines = file_content.lines().count();
        let digits_len = total_lines.to_string().len();
        let args = &self.args;

        file_content.lines().fold(
            String::with_capacity(file_content.len() + total_lines * 10),
            |mut result, line| {
                let mut line = line.to_string();

                if args.show_all_characters
                    || args.show_non_printing_characters_and_end_with_dollar
                    || args.show_non_printing_characters_except_tabs_and_eol
                {
                    let mut new_string = String::with_capacity(line.len() * 2);
                    line.chars().for_each(|c| match c {
                        '\0' => new_string.push_str("^@"),
                        '\x01'..='\x1F' => {
                            new_string.push_str(&format!("^{}", (c as u8 + 64) as char))
                        }
                        '\x7F' => new_string.push_str("^?"),
                        _ => new_string.push(c),
                    });
                    line = new_string;
                }

                if (args.show_tab_as_i || args.show_all_characters)
                    && !args.show_non_printing_characters_and_end_with_dollar
                    && !args.show_non_printing_characters_except_tabs_and_eol
                {
                    line = line.replace("\t", "^I");
                }

                if args.numbered_list_excluding_non_blank_lines && !line.is_empty() {
                    line = format!("{:width$}  {line}", line_count, width = digits_len + 2);
                    *line_count += 1;
                }

                if args.numbered_list_including_blank_lines
                    && !args.numbered_list_excluding_non_blank_lines
                {
                    line = format!("{:width$}  {line}", line_count, width = digits_len + 2);
                    *line_count += 1;
                }

                if (args.display_dollar_at_line_end
                    || args.show_all_characters
                    || args.show_non_printing_characters_and_end_with_dollar)
                    && !args.show_non_printing_characters_except_tabs_and_eol
                {
                    line.push('$');
                }

                result.push_str(&line);
                result.push('\n');
                result
            },
        )
    }
}
