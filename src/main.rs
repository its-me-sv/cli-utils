use clap::Parser;
use cli_utils::{
    args::{CliUtilsArgs, Tool},
    tools::{cat_handler, EchoParser},
};

fn main() {
    match CliUtilsArgs::parse().tool {
        Tool::Echo(echo_args) => print!("{}", EchoParser::new(echo_args).parse()),
        Tool::Cat(cat_args) => print!("{}", cat_handler(cat_args)),
        _ => println!("Something else"),
    }
}
