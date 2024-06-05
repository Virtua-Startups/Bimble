use std::path::Path;

pub fn check_folder(foldnm: &str) -> (bool, &Path) {
    if Path::new(foldnm).exists() {
        return (true, Path::new(foldnm));
    } else {
        return (false, Path::new(foldnm));
    }
}
