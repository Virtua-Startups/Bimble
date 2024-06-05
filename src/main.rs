#![allow(unused_assignments)]
use bimble::{check_folder, construct_newcd};
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

    let cpsepbynl = newcd.split("\n");
    let mut fndclr = false;
    let mut fns : Vec<String> = Vec::new();
    let mut fndclrbraces = false;
    let mut indexl = 1;
    for curline in cpsepbynl {
        let cds = curline.split_whitespace();
        for cd in cds{
            println!("{indexl} - {cd}");
            if cd == "ON" && fndclr != true{
                fndclr = true;
                
            }
            else if fndclr && cd != "{" && cd.contains("("){
                fns.push(cd.to_string());
            }
            else if !fndclrbraces && fndclr {
                fndclrbraces = true;
            }
            else if cd.is_empty(){
                continue;
            }
            else if cd == "(" {
                continue;
            }
            else if cd == ")"{
                continue;
            }
            else if cd == "{" && fndclrbraces{
                fndclrbraces = false;
            }
            else if cd == "}" && !fndclrbraces{
                fndclrbraces = true;
            }
            else {
                println!("{}{}{}{}","ERR AT LINE : ",indexl,"No keyword : ",cd);
            }
            indexl +=1;
        }
    }
}
