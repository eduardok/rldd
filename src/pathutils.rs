use std::path::Path;

pub fn get_path<P: AsRef<Path>>(path: &P) -> Option<String> {
    path.as_ref()
        .parent()
        .and_then(|s| s.to_str().and_then(|s| Some(s.to_string())))
}

pub fn get_name<P: AsRef<Path>>(path: &P) -> String {
    path.as_ref()
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string()
}

#[allow(dead_code)]
pub fn file_is_under_dir<P1: AsRef<Path>, P2: AsRef<Path>>(file: &P1, dir: &P2) -> bool
where
    Path: PartialEq<P2>,
{
    if let Some(parent) = file.as_ref().parent() {
        return parent == dir;
    }
    false
}
