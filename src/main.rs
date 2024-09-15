use clap::Parser;
use cli_utils::{
    args::{CliUtilsArgs, Tool},
    tools::{CatParser, EchoParser, LsParser},
};

fn main() {
    match CliUtilsArgs::parse().tool {
        Tool::Echo(echo_args) => print!("{}", EchoParser::new(echo_args).parse()),
        Tool::Cat(cat_args) => print!("{}", CatParser::new(cat_args).parse()),
        Tool::Ls(ls_args) => print!("{}", LsParser::new(ls_args).parse()),
        _ => println!("Something else"),
    }
}
