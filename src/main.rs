#![allow(unused_assignments)]
use bimble::{check_folder, cliip::run, construct_newcd};
use clearscreen::clear;
use colored::*;
use std::{
    env::{args, current_dir},
    fs,
    process::exit,
};

fn main() {
    let cmd: Vec<String> = args().collect();
    let mut newcd = String::new();
    if cmd.len() <= 1 {
        run();
    }
    let cmd = cmd
        .get(1)
        .expect("expected a project folder to be provided")
        .trim_matches('\'')
        .trim_matches('\"');
    let projfol = cmd;

    clear().unwrap();
    let curdir = current_dir().unwrap().to_string_lossy().to_string();
    let mut codefiles: Vec<String> = Vec::new();

    let projfol = if projfol == "." { &curdir } else { projfol };

    println!(
        "{}{}",
        "Project Folder : ".bold().green(),
        projfol.bold().bright_blue()
    );

    let projfol = check_folder(projfol);
    if !projfol.0 {
        println!("{}{:?}", "Project folder doesn't exist: ", projfol.1);
        exit(-1);
    }

    println!("{}{:?}", "Project Folder Exists: ", projfol.1);

    let fol = projfol.1.to_string_lossy().to_string();
    let files = fs::read_dir(&fol).unwrap_or_else(|err| {
        println!("{}{}", "Unable to read project dir: ", fol);
        println!("{}{}", "ERROR: ", err);
        exit(-1);
    });

    for file in files {
        match file {
            Ok(curfile) => {
                codefiles.push(curfile.file_name().into_string().unwrap());
            }
            Err(err) => {
                println!("{}{:?}", "ERROR READING FILE: ", err);
            }
        }
    }

    if !codefiles.contains(&"main.bb".to_string()) {
        println!(
            "{}{}",
            "ERROR 'main.bb' file not found in the project folder: "
                .bold()
                .red(),
            projfol.1.to_string_lossy().red()
        );
        exit(-1);
    } else {
        match fs::read_to_string(fol + "/main.bb") {
            Ok(cntn) => {
                newcd = construct_newcd(&cntn);
            }
            Err(err) => {
                println!(
                    "{}{}",
                    "ERROR READING 'main.bb' : ".bold().bright_red(),
                    err.to_string().red()
                );
                exit(-1);
            }
        }
    }
    let cpsepbynl = newcd.split('\n');
    let mut fndclr = false;
    let mut fns: Vec<String> = Vec::new();
    let mut fndclrbraces = false;
    let mut indexl = 1;
    let mut echonl = false;
    for curline in cpsepbynl {
        let cds: Vec<&str> = curline.split_whitespace().collect();
        for cd in cds {
            if cd == "ON" && !fndclr {
                fndclr = true;
            } else if fndclr {
                fns.push(cd.to_string());
                fndclr = false;
            } else if cd == "{" && !fndclrbraces {
                fndclrbraces = true;
            } else if cd == "}" && fndclrbraces {
                fndclrbraces = false;
                fndclr = false;
            } else if cd == ")" && !echonl {
                continue;
            } else if cd == ")" && echonl {
                echonl = false;
            } else if cd == "(" {
                continue;
            } else if cd == " " {
                continue;
            } else if cd == "echonl" {
                echonl = true;
            } else if cd == "(" && fndclr && echonl {
                continue;
            } else if cd == ";" {
                continue;
            } else {
                for ffns in fns.clone(){
                    if cd == ffns{
                        continue;
                    }
                }
                if echonl && cd == ")" {
                    echonl = false;
                    continue;
                } else if echonl {
                    continue;
                } else {
                    println!(
                        "err at line - '{}' - keyword - '{}' does not exists",
                        indexl, cd
                    );
                    exit(-1);
                }
            }
            indexl += 1;
        }
    }
    let css = [
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '{', '}', '[', ']',
        '|', '\\', ':', ';', '"', '\'', '<', '>', ',', '.', '?', '/', '`', '~',
    ];

    let mut hasmain = false;
    for fnn in fns.clone() {
        if fnn == "main" {
            if hasmain {
                println!(
                    "{}",
                    "MAIN FUNCTION DECLARED 2 TIMES OR MORE!!".bold().red()
                );
                exit(-1);
            }
            hasmain = true;
        }
    }
    for fnn in fns.clone() {
        for scc in css {
            if fnn == scc.to_string() {
                println!("ERR - FUNCTION NAMES CANT BE SPECIAL CHARACTERS\nFUNCTION NAME - {fnn}");
                exit(-1);
            }
        }
    }
    if !hasmain {
        println!("ERR - 'main' function not found in 'main.bb'");
    }
    println!("Code checking passed!\nprocceding to build!");
    let _datafile = projfol.1;
}
