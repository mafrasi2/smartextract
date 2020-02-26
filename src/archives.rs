use std::collections::HashSet;
use std::path::PathBuf;
use lazy_static;
use regex::Regex;

#[cfg(test)]
use matches::assert_matches;

#[derive(Debug)]
pub enum Archive {
    RAR(PathBuf),
    P7Z(PathBuf)
}

impl Archive {
    pub fn create(suffix: &str, path: PathBuf) -> Option<Self> {
        if suffix == "rar" {
            Some(Archive::RAR(path))
        } else if suffix == "7z" {
            Some(Archive::P7Z(path))
        } else {
            None
        }
    }

    pub fn path(& self) -> &PathBuf {
        match self {
            Archive::RAR(path) => &path,
            Archive::P7Z(path) => &path,
        }
    }

    fn is_supported_type(suffix: &str) -> bool {
        lazy_static::lazy_static! {
            static ref ARCHIVE_TYPES: HashSet<&'static str> = ["rar", "7z"].iter().cloned().collect();
        }
        ARCHIVE_TYPES.contains(suffix)
    }
}

pub fn detect_archive(path: PathBuf) -> Option<Archive> {
    lazy_static::lazy_static! {
        static ref INFIX_PART_RE: Regex = Regex::new("^part\\d+$").unwrap();
        static ref INFIX_FIRST_PART_RE: Regex = Regex::new("^part0*1$").unwrap();
        static ref SUFFIX_PART_RE: Regex = Regex::new("^\\d+$").unwrap();
        static ref SUFFIX_FIRST_PART_RE: Regex = Regex::new("^0*1$").unwrap();
    }

    let fname = match path.file_name() {
        Some(fname) => fname,
        None => return None
    };
    let fname = fname.to_string_lossy().to_lowercase();

    let parts: Vec<_> = fname.rsplit(".").collect();
    if Archive::is_supported_type(parts[0]) {
        if parts.len() <= 1 {
            Archive::create(parts[0], path)
        } else if INFIX_FIRST_PART_RE.is_match(parts[1]) || !INFIX_PART_RE.is_match(parts[1]) {
            Archive::create(parts[0], path)
        } else {
            None
        }
    } else if parts.len() > 1 && Archive::is_supported_type(parts[1]) {
        if SUFFIX_FIRST_PART_RE.is_match(parts[0]) || !SUFFIX_PART_RE.is_match(parts[0]) {
            Archive::create(parts[1], path)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn delete_archive(path: &PathBuf) {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn archive_names() {
        assert_matches!(detect_archive(PathBuf::from(".rar")), Some(Archive::RAR(_)));
        assert_matches!(detect_archive(PathBuf::from("abc.rar")), Some(Archive::RAR(_)));
        assert_matches!(detect_archive(PathBuf::from("abc.rar.001")), Some(Archive::RAR(_)));
        assert_matches!(detect_archive(PathBuf::from("abc.rar.002")), None);
        assert_matches!(detect_archive(PathBuf::from("a.part1.rar")), Some(Archive::RAR(_)));
        assert_matches!(detect_archive(PathBuf::from("a.part001.rar")), Some(Archive::RAR(_)));
        assert_matches!(detect_archive(PathBuf::from("a.part2.rar")), None);

        assert_matches!(detect_archive(PathBuf::from(".7z")), Some(Archive::P7Z(_)));
        assert_matches!(detect_archive(PathBuf::from("a.7z.001")), Some(Archive::P7Z(_)));
        assert_matches!(detect_archive(PathBuf::from("a.7z.002")), None);
        assert_matches!(detect_archive(PathBuf::from("a.7z.010")), None);
        assert_matches!(detect_archive(PathBuf::from("a.part01.7z")), Some(Archive::P7Z(_)));
    }
}