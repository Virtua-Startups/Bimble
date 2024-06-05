use std::path::Path;

pub fn check_folder(foldnm: &str) -> (bool, &Path) {
    if Path::new(foldnm).exists() {
        return (true, Path::new(foldnm));
    } else {
        return (false, Path::new(foldnm));
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
            '\n' => {
                newcd.push(c);
                prevchr = None; // Reset prevchr after a newline
            }
            '(' => {
                newcd.push(' ');
                newcd.push(c);
            }
            ')' => {
                newcd.push(' ');
                newcd.push(c);
            }
            '{' => {
                newcd.push(' ');
                newcd.push(c);
            }
            '}' => {
                newcd.push(' ');
                newcd.push(c);
            }
            ';' => {
                newcd.push(' ');
                newcd.push(c);
            }
            _ => {
                //newcd.push(' ');
                newcd.push(c);
            }
        }
        if c != '\n' {
            prevchr = Some(c);
        }
    }

    newcd
}
