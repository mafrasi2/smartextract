use std::collections::HashSet;
use std::path::PathBuf;
use lazy_static;
use regex::Regex;

pub enum UnpackError {
    NoPassword,
    Incomplete,
    Unknown,
}


pub fn is_archive(path: &PathBuf) -> bool {
    lazy_static::lazy_static! {
        static ref INFIX_PART_RE: Regex = Regex::new("^part\\d+$").unwrap();
        static ref INFIX_FIRST_PART_RE: Regex = Regex::new("^part0*1$").unwrap();
        static ref SUFFIX_PART_RE: Regex = Regex::new("^\\d+$").unwrap();
        static ref SUFFIX_FIRST_PART_RE: Regex = Regex::new("^0*1$").unwrap();

        static ref ARCHIVE_NAMES: HashSet<&'static str> = ["rar", "7z"].iter().cloned().collect();
    }

    let fname = match path.file_name() {
        Some(fname) => fname,
        None => return false
    };
    let fname = fname.to_string_lossy().to_lowercase();

    let parts: Vec<_> = fname.rsplit(".").collect();
    if ARCHIVE_NAMES.contains(parts[0]) {
        if parts.len() <= 1 {
            true
        } else {
            INFIX_FIRST_PART_RE.is_match(parts[1]) || !INFIX_PART_RE.is_match(parts[1])
        }
    } else if parts.len() > 1 && ARCHIVE_NAMES.contains(parts[1]) {
        SUFFIX_FIRST_PART_RE.is_match(parts[0]) || !SUFFIX_PART_RE.is_match(parts[0])
    } else {
        false
    }
}

pub fn try_unpack(path: &PathBuf) -> Result<(), UnpackError> {
    println!("{}", path.as_os_str().to_string_lossy());
    Err(UnpackError::Unknown)
}

pub fn delete_archive(path: &PathBuf) {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn archive_names() {
        assert!(is_archive(&PathBuf::from(".rar")));
        assert!(is_archive(&PathBuf::from("abc.rar")));
        assert!(is_archive(&PathBuf::from("abc.rar.001")));
        assert!(!is_archive(&PathBuf::from("abc.rar.002")));
        assert!(is_archive(&PathBuf::from("a.part1.rar")));
        assert!(is_archive(&PathBuf::from("a.part001.rar")));
        assert!(!is_archive(&PathBuf::from("a.part2.rar")));

        assert!(is_archive(&PathBuf::from(".7z")));
        assert!(is_archive(&PathBuf::from("a.7z.001")));
        assert!(is_archive(&PathBuf::from("a.part01.rar")));
        assert!(!is_archive(&PathBuf::from("a.7z.002")));
        assert!(!is_archive(&PathBuf::from("a.7z.010")));
    }
}