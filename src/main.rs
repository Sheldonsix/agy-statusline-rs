use std::{
    io::{self, Read},
    process::exit,
};

use agy_statusline_rs::{cli::Args, render_statusline_from_input};
use clap::Parser;

fn main() {
    let args = Args::parse();

    let mut input_data = Vec::new();
    io::stdin().read_to_end(&mut input_data).unwrap_or(0);

    match render_statusline_from_input(&input_data, &args) {
        Ok(Some(output)) => println!("{}", output),
        Ok(None) => exit(0),
        Err(error) => {
            println!("\x1b[31m{}\x1b[0m", error);
            exit(1);
        }
    }
}
