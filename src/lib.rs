use std::path::Path;

#[allow(unused_assignments)]
pub fn checkcd(code: &str) -> (bool, &str) {
    let mut fndclr = false;
    let mut fndclrbraces = false;
    let mut echonl = false;
    let mut fns: Vec<String> = Vec::new();
    let cdd = code.split_whitespace();
    let mut txtt = String::new();
    let (mut fq, mut fb) = (false, false);

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
                println!("{}", txtt.trim()); // Print the collected text when echonl ends
                txtt.clear();
                (true, "OK")
            }
            txt if echonl => {
                let ii: Vec<char> = txt.chars().collect();
                let lenn = ii.len();
                let mut curlenn = 0;
                for i in ii {
                    if i == '(' && !fb {
                        fb = true;
                        curlenn += 1;
                        continue;
                    } else if i == '(' && fb {
                        txtt.push(i);
                        curlenn += 1;
                    } else if i == '"' && !fq {
                        fq = true;
                        curlenn += 1;
                        continue;
                    } else if i == '"' && fq && curlenn == lenn - 1 {
                        // Skip the last quotation mark
                        break;
                    } else if i == '"' && fq {
                        fq = false;
                        curlenn += 1;
                    } else {
                        txtt.push(i);
                        curlenn += 1;
                    }
                }
                txtt.push(' '); // Preserve spaces between words
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
    let mut in_quotes = false;

    for c in content.chars() {
        match c {
            ' ' if !in_quotes => {
                if prevchr != Some(' ') {
                    newcd.push(c);
                }
            }
            '"' => {
                in_quotes = !in_quotes;
                newcd.push(c);
            }
            '\'' | '\\' | '(' | ')' | '{' | '}' | ';' => {
                if !in_quotes {
                    newcd.push(' ');
                }
                newcd.push(c);
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
