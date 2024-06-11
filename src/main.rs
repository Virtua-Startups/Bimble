use bimble::cliip::run;
use colored::*;
use std::{env::args, fs::File, io::Read, path::Path, process::exit};

fn main() {
    let total_checking_times = 4;
    let mut cuchecks = 0;
    let f: Vec<String> = args().collect();
    if f.len() <= 1 {
        run();
    } else if f.len() > 2 {
        eprintln!(
            "{}",
            "Error - Only 1 project can be compiled at a time!"
                .bold()
                .red()
        );
        exit(1);
    }

    let f = f.get(1).unwrap().trim_matches('\'').trim_matches('\"');
    let projfold = Path::new(f);

    if !projfold.exists() {
        eprintln!(
            "{} {:?}",
            "Error: Project Folder Does Not Exist:".bold().red(),
            projfold
        );
        exit(1);
    }

    let mfff = format!("{}/main.bb", f);

    let mut isfunc = false;
    let mut fns: Vec<String> = Vec::new();
    let mut undefined_calls: Vec<String> = Vec::new();

    while cuchecks < total_checking_times {
        cuchecks += 1;
        println!("Opening file: {}", mfff); // Debug statement
        let mffff = File::open(&mfff); // Re-open the file on each iteration

        match mffff {
            Ok(mut mf) => {
                let mut code = String::new();
                match mf.read_to_string(&mut code) {
                    Ok(_) => {
                        println!("Read code: \n{}", code); // Debug statement
                        let cdp = code.split('\n');
                        for cd in cdp {
                            if !cd.contains("echonl") {
                                let codes = cd.split_whitespace();
                                for token in codes {
                                    dbg!(&token, &isfunc); // Debug statement
                                    if token == "ON" {
                                        println!("Found 'ON' token"); // Debug statement
                                        isfunc = true;
                                    } else if isfunc {
                                        match token.find('(') {
                                            Some(index) => {
                                                let re = &token[..index];
                                                if !fns.contains(&re.to_string()) {
                                                    println!("Adding function to list: {}", re); // Debug statement
                                                    fns.push(re.to_string());
                                                }
                                                isfunc = false;
                                            }
                                            None => {
                                                eprintln!(
                                                    "Error: Invalid function declaration: {}",
                                                    token
                                                );
                                                exit(1);
                                            }
                                        }
                                    } else if token == "}" {
                                        continue;
                                    } else {
                                        println!("Checking function call: {}", token); // Debug statement
                                        match token.find('(') {
                                            Some(index) => {
                                                let re = &token[..index];
                                                println!("{re}");
                                                if !fns.contains(&re.to_string()) && re != "main" {
                                                    if cuchecks == total_checking_times {
                                                        undefined_calls.push(re.to_string());
                                                    }
                                                }
                                            }
                                            None => {
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                let codes = cd.trim();
                                if !codes.contains(";") {
                                    eprintln!("Error: Missing ';' at end of line: {}", cd);
                                    exit(-1);
                                }
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!(
                            "{} {}",
                            "Error: Unable to read code from main.bb".bold().red(),
                            format!("Error: {}", err)
                        );
                        exit(1);
                    }
                }
                println!("Completed check iteration: {}", cuchecks); // Debug statement
            }
            Err(err) => {
                eprintln!(
                    "{} {}",
                    "Error: Cannot Find 'main.bb' file".bold().red(),
                    format!("Error: {}", err)
                );
                exit(1);
            }
        }
    }

    // Debugging output to show collected function names
    println!("Collected function names: {:?}", fns);

    // After all checks, report any undefined function calls
    if !undefined_calls.is_empty() {
        eprintln!("Undefined function calls found:");
        for call in undefined_calls {
            eprintln!("  {}", call);
        }
        exit(1);
    }
}
