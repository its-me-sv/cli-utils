use std::fs;

use crate::args::CatArgs;

pub fn cat_handler(args: CatArgs) -> String {
    let CatArgs {
        file_names,
        show_all_characters,
        numbered_list_excluding_non_blank_lines,
        show_non_printing_characters_and_end_with_dollar,
        display_dollar_at_line_end,
        numbered_list_including_blank_lines,
        show_tab_as_i,
        show_non_printing_characters_except_tabs_and_eol,
        ..
    } = args;

    match file_names {
        Some(file_names) => {
            let mut line_count = 1;

            file_names
                .iter()
                .map(|file_name| {
                    match fs::read_to_string(file_name) {
                        Ok(file_content) => {
                            let file_contents = file_content.lines().collect::<Vec<&str>>();
                            let total_lines = file_contents.len();
                            let digits_len = total_lines.to_string().len();

                            file_contents
                                .into_iter()
                                .map(|line| {
                                    let mut line = format!("{line}\n");

                                    // Replacing non-printable characters - A, e, v
                                    if show_all_characters
                                        || show_non_printing_characters_and_end_with_dollar
                                        || show_non_printing_characters_except_tabs_and_eol
                                    {
                                        line = line
                                            .chars()
                                            .map(|c| match c {
                                                '\0' => "^@".to_string(),
                                                '\x01'..='\x1F' => {
                                                    format!("^{}", (c as u8 + 64) as char)
                                                }
                                                '\x7F' => "^?".to_string(),
                                                _ => c.to_string(),
                                            })
                                            .collect();
                                    }

                                    // Replacing tabs - T
                                    if (show_tab_as_i || show_all_characters)
                                        && !show_non_printing_characters_and_end_with_dollar
                                        && !show_non_printing_characters_except_tabs_and_eol
                                    {
                                        line = line.replace("\t", "^I");
                                    }

                                    // Numbering non-blank lines - b
                                    if numbered_list_excluding_non_blank_lines && !line.is_empty() {
                                        line = format!(
                                            "{:width$}  {line}",
                                            line_count,
                                            width = digits_len + 2
                                        );
                                        line_count += 1;
                                    }

                                    // Numbering all lines - n
                                    if numbered_list_including_blank_lines
                                        && !numbered_list_excluding_non_blank_lines
                                    {
                                        line = format!(
                                            "{:width$}  {line}",
                                            line_count,
                                            width = digits_len + 2
                                        );
                                        line_count += 1;
                                    }

                                    // Replacing new line - E
                                    if (display_dollar_at_line_end
                                        || show_all_characters
                                        || show_non_printing_characters_and_end_with_dollar)
                                        && !show_non_printing_characters_except_tabs_and_eol
                                    {
                                        line = line.replace("\n", "$\n");
                                    }

                                    line
                                })
                                .collect()
                        }
                        Err(_) => format!("cat: {}: No such file or directory", file_name),
                    }
                })
                .collect()
        }
        None => String::new(),
    }
}
