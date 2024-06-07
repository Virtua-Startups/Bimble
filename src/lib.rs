use std::{io::{stdout, Write}, path::Path};

#[allow(unused_assignments)]
pub fn checkcd(cdddd: &str) -> (bool, &str) {
    let mut fndclr = false;
    let mut fndclrbraces = false;
    let mut echonl = false;
    let mut fns: Vec<String> = Vec::new();
    let cdd = cdddd.split_whitespace();
    let mut txtt = String::new();

    for cd in cdd {
        let result = match cd {
            "ON" if !fndclr => {
                fndclr = true;
                (true, "OK")
            }
            "{" if !fndclrbraces => {
                fndclrbraces = true;
                (true, "OK")
            }
            "}" if fndclrbraces => {
                fndclrbraces = false;
                fndclr = false;
                (true, "OK")
            }
            ")" | "(" | " " if !echonl => (true, "OK"),
            "echonl" => {
                echonl = true;
                fndclr = false; // reset fndclr after processing echonl
                (true, "OK")
            }
            ")" if echonl => {
                echonl = false;
                (true,"OK")
            }
            txt if echonl => {
                let chars: Vec<char> = txt.chars().collect();
                let (mut fb, mut fq) = (false, false);
                txtt.clear();
                for i in chars {
                    if i == '(' && fb {
                        txtt.push(i);
                    } else if i == '(' && !fb {
                        fb = true;
                        continue;
                    } else if i == '\"' && fq {
                        txtt.push(i);
                    } else if i == '\"' && !fq {
                        fq = true;
                        continue;
                    } else {
                        txtt.push(i);
                    }
                }
                print!("{txtt}");
                stdout().flush().unwrap();
                (true, "OK")
            }
            _ if fndclr => {
                fns.push(cd.to_string());
                fndclr = false;
                (true, "OK")
            }
            ";" => (true, "OK"),
            _ => (false, "ERR"),
        };

        // Return early if any result is an error
        if !result.0 {
            return result;
        }
    }
    (true, "OK") // Default return if all commands are processed correctly
}

#[allow(unused_assignments)]
pub mod cliip;

pub fn check_folder(foldnm: &str) -> (bool, &Path) {
    if Path::new(foldnm).exists() {
        (true, Path::new(foldnm))
    } else {
        (false, Path::new(foldnm))
    }
}

pub fn construct_newcd(content: &str) -> String {
    let mut newcd = String::new();
    let mut prevchr: Option<char> = None;

    for c in content.chars() {
        match c {
            ' ' => {
                if prevchr != Some(' ') {
                    newcd.push(c);
                }
            }
            '\"' => {
                newcd.push(' ');
                newcd.push(c);
            }
            '\'' => {
                newcd.push(' ');
                newcd.push(c);
            }
            '\\' => {
                newcd.push(' ');
                newcd.push(c);
            }
            '\n' => {
                newcd.push(c);
                prevchr = None; // Reset prevchr after a newline
            }
            '(' | ')' | '{' | '}' | ';' => {
                newcd.push(' ');
                newcd.push(c);
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