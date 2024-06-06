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
    let kr = true;
    while kr {
        print!("> ");
        match stdout().flush() {
            Ok(_) => {
                let mut cmd = String::new(); // Move this here to reset `cmd` each iteration
                match stdin().read_line(&mut cmd) {
                    Ok(_) => {
                        let cmd = cmd.trim();
                        let cmd = &construct_newcd(cmd);
                        if cmd == "exit!" {
                            exit(-1);
                        } else if cmd == "clear!" {
                            clear().unwrap();
                        } else {
                            let ccd = checkcd(cmd);
                            if ccd.0 {
                                println!("OK");
                            } else {
                                println!(
                                    "{}{}{}",
                                    "ERR - THE COMMAND : ".red(),
                                    cmd.to_string().red(),
                                    " : DOES NOT EXISTS PLEASE CHECK".red()
                                );
                            }
                        }
                    }
                    Err(err) => {
                        println!(
                            "{}{}",
                            "Unable to read input\nERR - ".red(),
                            err.to_string().bright_red()
                        );
                        exit(-1);
                    }
                }
            }
            Err(err) => {
                println!(
                    "{}{}",
                    "Unable to flush the terminal...please try again\nerr - ".red(),
                    err.to_string().bright_red()
                );
                exit(-1);
            }
        }
    }
}
