use clap::Parser;
use cli_utils::{
    args::{CliUtilsArgs, Tool},
    tools::echo_handler,
};

fn main() {
    let args = CliUtilsArgs::parse();

    match args.tool {
        Tool::Echo(echo_args) => print!("{}", echo_handler(echo_args)),
        _ => println!("Something else"),
    }
}
