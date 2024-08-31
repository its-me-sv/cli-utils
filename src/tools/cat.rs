use std::fs;

use crate::args::CatArgs;

pub struct CatParser {
    args: CatArgs,
    line_count: u128,
    last_line_was_empty: bool,
}

impl CatParser {
    pub fn new(args: CatArgs) -> Self {
        Self {
            args,
            line_count: 1,
            last_line_was_empty: false,
        }
    }

    pub fn parse(&mut self) -> String {
        self.args
            .file_names
            .take() // Taking out from self, make sure to put it back after using
            .map_or(String::with_capacity(0), |file_names| {
                let estimated_size = file_names
                    .iter()
                    .map(|name| fs::metadata(name).map_or(0, |m| m.len() as usize))
                    .sum();
                let mut result = String::with_capacity(estimated_size);

                let parsed_files = file_names.iter().map(|file_name| {
                    fs::read_to_string(file_name).map_or(
                        format!("cat: {}: No such file or directory\n", file_name),
                        |file_content| self.parse_file_contents(file_content),
                    )
                });
                result.extend(parsed_files);

                // Assigning back the taken value take out from self
                self.args.file_names = Some(file_names);

                result
            })
    }

    fn parse_file_contents(&mut self, file_content: String) -> String {
        let total_lines = file_content.lines().count();
        let largest_digit_width = total_lines.to_string().len();

        let estimated_size = file_content.len() + total_lines * 10;
        let mut result = String::with_capacity(estimated_size);

        let parsed_lines = file_content
            .lines()
            .map(|line| self.parse_line(line, largest_digit_width));
        result.extend(parsed_lines);

        result
    }

    fn parse_line(&mut self, line: &str, largest_digit_width: usize) -> String {
        let mut line = line.to_string();
        let curr_line_is_empty = line.is_empty();

        if curr_line_is_empty {
            if self.args.squeeze_adjacent_blank_lines && self.last_line_was_empty {
                self.last_line_was_empty = false;
                return line;
            }
            self.last_line_was_empty = true;
        }

        if self.args.show_all_characters
            || self.args.show_non_printing_characters_and_end_with_dollar
            || self.args.show_non_printing_characters_except_tabs_and_eol
        {
            let mut new_string = String::with_capacity(line.len() * 2);
            line.chars().for_each(|c| match c {
                '\0' => new_string.push_str("^@"),
                '\x01'..='\x1F' => new_string.push_str(&format!("^{}", (c as u8 + 64) as char)),
                '\x7F' => new_string.push_str("^?"),
                _ => new_string.push(c),
            });
            line = new_string;
        }

        if (self.args.show_tab_as_i || self.args.show_all_characters)
            && !self.args.show_non_printing_characters_and_end_with_dollar
            && !self.args.show_non_printing_characters_except_tabs_and_eol
        {
            line = line.replace("\t", "^I");
        }

        if self.args.numbered_list_excluding_non_blank_lines && !line.is_empty() {
            line = format!(
                "{:width$}  {line}",
                self.line_count,
                width = largest_digit_width + 2
            );
            self.line_count += 1;
        }

        if self.args.numbered_list_including_blank_lines
            && !self.args.numbered_list_excluding_non_blank_lines
        {
            line = format!(
                "{:width$}  {line}",
                self.line_count,
                width = largest_digit_width + 2
            );
            self.line_count += 1;
        }

        if (self.args.display_dollar_at_line_end
            || self.args.show_all_characters
            || self.args.show_non_printing_characters_and_end_with_dollar)
            && !self.args.show_non_printing_characters_except_tabs_and_eol
        {
            line.push('$');
        }

        line.push('\n');
        line
    }
}
