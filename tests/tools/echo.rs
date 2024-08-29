use cli_utils::{args::EchoArgs, tools::EchoParser};

#[test]
fn echo_with_no_options() {
    let input = EchoArgs::new(Some(vec!["Hello, World!".to_string()]), true, false, false);
    assert_eq!("Hello, World!\n", EchoParser::new(input).parse());
}

#[test]
fn echo_with_omit_newline() {
    let input = EchoArgs::new(Some(vec!["Hello, World!".to_string()]), true, false, true);
    assert_eq!("Hello, World!", EchoParser::new(input).parse());
}

#[test]
fn echo_with_escape_characters_enabled() {
    let input = EchoArgs::new(Some(vec!["Hello\\nWorld".to_string()]), true, true, false);
    assert_eq!("Hello\nWorld\n", EchoParser::new(input).parse());
}

#[test]
fn echo_with_escape_characters_disabled() {
    let input = EchoArgs::new(Some(vec!["Hello\\nWorld".to_string()]), true, false, false);
    assert_eq!("Hello\\nWorld\n", EchoParser::new(input).parse());
}

#[test]
fn echo_with_escape_characters_enabled_with_omit_newline() {
    let input = EchoArgs::new(Some(vec!["Hello\\tWorld".to_string()]), true, true, true);
    assert_eq!("Hello\tWorld", EchoParser::new(input).parse());
}

#[test]
fn echo_with_escape_characters_disabled_with_omit_newline() {
    let input = EchoArgs::new(Some(vec!["Hello\\tWorld".to_string()]), true, false, true);
    assert_eq!("Hello\\tWorld", EchoParser::new(input).parse());
}

#[test]
fn echo_with_mixed_content_with_escape_characters_enabled() {
    let input = EchoArgs::new(
        Some(vec!["Line1\\nLine2\\tTabbed".to_string()]),
        true,
        true,
        false,
    );
    assert_eq!("Line1\nLine2\tTabbed\n", EchoParser::new(input).parse());
}

#[test]
fn echo_with_escaped_backslash_and_quotes() {
    let input = EchoArgs::new(
        Some(vec!["C:\\\\Path\\\"to\\\"file".to_string()]),
        true,
        true,
        false,
    );
    assert_eq!("C:\\Path\"to\"file\n", EchoParser::new(input).parse());
}

#[test]
fn echo_with_empty_input() {
    let input = EchoArgs::new(None, true, false, false);
    assert_eq!("\n", EchoParser::new(input).parse());
}

#[test]
fn echo_with_multiple_strings() {
    let input = EchoArgs::new(
        Some(vec!["Hello,".to_string(), "World!".to_string()]),
        true,
        false,
        false,
    );
    assert_eq!("Hello, World!\n", EchoParser::new(input).parse());
}

#[test]
fn echo_with_multiple_strings_and_escape_enabled() {
    let input = EchoArgs::new(
        Some(vec!["Hello\\n".to_string(), "World".to_string()]),
        true,
        true,
        false,
    );
    assert_eq!("Hello\n World\n", EchoParser::new(input).parse());
}

#[test]
fn echo_with_multiple_spaces() {
    let input = EchoArgs::new(
        Some(vec![
            "This".to_string(),
            "is".to_string(),
            " a".to_string(),
            " test".to_string(),
        ]),
        true,
        false,
        false,
    );
    assert_eq!("This is a test\n", EchoParser::new(input).parse());
}
