use bimble::check_folder;
use clearscreen::clear;
use colored::*;
use std::{
    env::{args, current_dir},
    fs,
    process::exit,
};

fn main() {
    let cmd: Vec<String> = args().collect();
    let mut projfol = cmd.get(1).unwrap_or_else(|| {
        eprintln!(
            "{}{}",
            "Error: ".red().bold(),
            "No project folder provided.".bold()
        );
        exit(-1);
    });

    clear().unwrap();
    let curdir = current_dir().unwrap().to_string_lossy().to_string();
    let mut codefiles: Vec<String> = Vec::new();

    if projfol == "." {
        projfol = &curdir;
    }

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

    let fol = projfol.1;
    let fol = fol.to_string_lossy().to_string();
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
            Err(errr) => {
                println!("{}{:?}", "ERROR READING FILE: ", errr);
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
                let newcd = construct_newcd(&cntn);
                println!("{newcd}");
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
}

fn construct_newcd(content: &str) -> String {
    let mut newcd = String::new();
    let mut prevchr: Option<char> = None;

    for c in content.chars() {
        match c {
            ' ' => {
                if prevchr != Some(' ') {
                    newcd.push(c);
                }
            }
            '\n' => {
                newcd.push(c);
                prevchr = None; // Reset prevchr after a newline
            }
            _ => {
                newcd.push(c);
            }
        }
        if c != '\n' {
            prevchr = Some(c);
        }
    }

    newcd
}
