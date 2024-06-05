use bimble::check_folder;
use clearscreen::clear;
use colored::*;
use std::{
    env::{args, current_dir},
    fs::{self},
    process::exit,
};
fn main() {
    let cmd: Vec<String> = args().collect();
    let mut projfol = cmd.get(1).unwrap();
    clear().unwrap();
    let curdir = &current_dir().unwrap().to_string_lossy().to_string();
    let mut codefiles: Vec<String> = Vec::new();
    if projfol == "." {
        projfol = curdir;
    }
    println!(
        "{}{}",
        "Project Folder : ".bold().green(),
        projfol.bold().bright_blue()
    );

    let projfol = check_folder(&projfol);
    match projfol.0 {
        true => {
            println!("{}{:?}", "Project Folder Exists : ", projfol.1);
        }
        false => {
            println!("{}{:?}", "project folder doesnt exists : ", projfol.1);
            exit(-1);
        }
    }
    let fol = projfol.1;
    let fol = fol.to_string_lossy().to_string();
    let files = fs::read_dir(fol.clone());
    match files {
        Ok(files) => {
            for file in files {
                match file {
                    Ok(curfile) => {
                        codefiles.push(curfile.file_name().into_string().unwrap());
                    }
                    Err(errr) => {
                        println!("{}{:?}", "ERROR READING FILE : ", errr);
                    }
                }
            }
        }
        Err(err) => {
            println!("{}{}", "Unable to read project dir : ", fol);
            println!("{}{}", "ERROR :", err);
            exit(-1);
        }
    }
    let mut endofcodefiles = 1;
    for i in codefiles.clone() {
        println!("{i} - {endofcodefiles}");
        if i == "main.bb" {
            endofcodefiles += 1;
            continue;
        } else if i != "main.bb" && endofcodefiles == codefiles.clone().len() {
            println!(
                "{}{}",
                "ERROR 'main.bb' file not found in the project folder : "
                    .bold()
                    .red(),
                projfol.1.to_string_lossy()
            );
            exit(-1);
        } else {
            endofcodefiles += 1;
        }
    }
}
