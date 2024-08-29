use clap::Parser;
use cli_utils::{
    args::{CliUtilsArgs, Tool},
    tools::{CatParser, EchoParser},
};

fn main() {
    match CliUtilsArgs::parse().tool {
        Tool::Echo(echo_args) => print!("{}", EchoParser::new(echo_args).parse()),
        Tool::Cat(cat_args) => print!("{}", CatParser::new(cat_args).parse()),
        _ => println!("Something else"),
    }
}
