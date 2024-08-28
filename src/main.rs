use clap::Parser;
use cli_utils::{
    args::{CliUtilsArgs, Tool},
    tools::{cat_handler, echo_handler},
};

fn main() {
    let args = CliUtilsArgs::parse();

    match args.tool {
        Tool::Echo(echo_args) => print!("{}", echo_handler(echo_args)),
        Tool::Cat(cat_args) => print!("{}", cat_handler(cat_args)),
        _ => println!("Something else"),
    }
}
