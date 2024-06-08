use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use clearscreen::clear;
use colored::Colorize;

use crate::{checkcd, construct_newcd};

pub fn run() {
    let vernum: &str = "v0.1";

    clear().unwrap();
    println!(
        "{}{}{}",
        "The Bimble Programming Language ".bold().bright_green(),
        vernum.bold().bright_green().blink(),
        "\nType Bimble or 'bb' code or\ntype 'help()' for info"
            .bold()
            .bright_green()
    );

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut cmd = String::new();
        stdin().read_line(&mut cmd).unwrap();
        let cmd = cmd.trim();
        let cmd = construct_newcd(cmd);

        match cmd.as_str() {
            "exit!" => exit(0),
            "clear!" => clear().unwrap(),
            _ => {
                let ccd = checkcd(&cmd);
                if !ccd.0 {
                    println!(
                        "\n{}{}{}",
                        "ERR - THE COMMAND : ".red(),
                        cmd.to_string().red(),
                        " : DOES NOT EXIST. PLEASE CHECK".red()
                    );
                }
            }
        }
    }
}
