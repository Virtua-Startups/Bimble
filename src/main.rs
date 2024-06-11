use bimble::cliip::run;
use colored::*;
use std::{env::args, fs::File, io::Read, path::Path, process::exit};

fn main() {
    let total_checking_times = 2;
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
        let mffff = File::open(&mfff); // Re-open the file on each iteration

        match mffff {
            Ok(mut mf) => {
                let mut code = String::new();
                match mf.read_to_string(&mut code) {
                    Ok(_) => {
                        let cdp = code.split('\n');
                        for cd in cdp {
                            if !cd.contains("echonl") {
                                let codes = cd.split_whitespace();
                                for token in codes {
                                    if token == "ON" {
                                        isfunc = true;
                                    } else if isfunc {
                                        match token.find('(') {
                                            Some(index) => {
                                                let re = &token[..index];
                                                fns.push(re.to_string());
                                            }
                                            None => {
                                                eprintln!(
                                                    "Error: Invalid function declaration: {}",
                                                    token
                                                );
                                                exit(1);
                                            }
                                        }
                                        isfunc = false;
                                    } else if token == "}" {
                                        continue;
                                    } else {
                                        let mut found = false;
                                        for i in &fns {
                                            if let Some(index) = token.find('(') {
                                                let re = &token[..index];
                                                if re == i {
                                                    found = true;
                                                    break;
                                                }
                                            }
                                        }
                                        if !found {
                                            undefined_calls.push(token.to_string());
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
                cuchecks += 1;
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

    // After all checks, report any undefined function calls
    if !undefined_calls.is_empty() {
        eprintln!("Undefined function calls found:");
        for call in undefined_calls {
            eprintln!("  {}", call);
        }
        exit(1);
    }
}
