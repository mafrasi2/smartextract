use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::PathBuf;
use lazy_static;
use regex::Regex;

#[cfg(test)]
use matches::assert_matches;

#[derive(Debug, Clone)]
pub enum ArchiveKind {
    RAR,
    P7Z,
}

#[derive(Debug)]
pub struct Archive {
    pub basename: OsString,
    pub parts: Vec<PathBuf>,
    pub kind: ArchiveKind,
}

impl Archive {
    pub fn create(basename: OsString, parts: Vec<PathBuf>, kind: ArchiveKind) -> Option<Self> {
        Some(Archive {
            basename,
            parts,
            kind
        })
    }

    pub fn delete(&self) -> io::Result<()> {
        let mut ret = Ok(());
        for part in &self.parts {
            let res = fs::remove_file(part);
            if res.is_err() {
                ret = res;
            }
        }
        ret
    }
}

impl ArchiveKind {
    fn from_suffix(suffix: &str) -> Option<&ArchiveKind> {
        lazy_static::lazy_static! {
            static ref ARCHIVE_TYPES: HashMap<&'static str, ArchiveKind> = [
                ("rar", ArchiveKind::RAR), ("7z", ArchiveKind::P7Z)
            ].iter().cloned().collect();
        }
        ARCHIVE_TYPES.get(suffix)
    }
}

pub fn detect_archive(mut path: PathBuf) -> Option<Archive> {
    lazy_static::lazy_static! {
        static ref INFIX_PART_RE: Regex = Regex::new("^part\\d+$").unwrap();
        static ref INFIX_FIRST_PART_RE: Regex = Regex::new("^part(0*)1$").unwrap();
        static ref SUFFIX_FIRST_PART_RE: Regex = Regex::new("^(0*)1$").unwrap();
    }

    let fname = match path.file_name() {
        Some(fname) => fname,
        None => return None
    };
    let fname_lc = fname.to_string_lossy().to_lowercase();

    let mut parts_rev: Vec<_> = fname_lc.rsplit(".").collect();
    if let Some(kind) = ArchiveKind::from_suffix(parts_rev[0]) {
        // for example foo.part1.rar
        if parts_rev.len() <= 1 {
            Archive::create(
                "unnamed".into(),
                vec![path],
                kind.clone()
            )
        } else if let Some(caps) = INFIX_FIRST_PART_RE.captures(parts_rev[1]) {
            let mut parts_orig: Vec<String> = fname.to_string_lossy()
                .split(".")
                .map(|s| s.to_string())
                .collect();
            let num_base_parts = parts_orig.len() - 2;
            let basename = parts_orig[..num_base_parts].iter()
                .map(|s| &**s)
                .collect::<Vec<_>>()
                .join(".");

            let mut archive_parts = vec![];
            let padding = caps[1].len() + 1;
            let unicode_name = parts_rev.join(".");
            if fname != &unicode_name[..] {
                return None;
            }
            let mut ctr: usize = 1;
            while path.exists() {
                archive_parts.push(path.clone());
                ctr += 1;
                parts_orig[1] = format!("part{:0padding$}", ctr, padding = padding);
                path.set_file_name(parts_orig.join("."));
            }

            Archive::create(basename.into(), archive_parts, kind.clone())
        } else if !INFIX_PART_RE.is_match(parts_rev[1]) {
            Archive::create(
                parts_rev[1..].join(".").into(),
                vec![path],
                kind.clone()
            )
        } else {
            None
        }
    } else if parts_rev.len() > 1 {
        // for example foo.rar.001
        if let Some(kind) = ArchiveKind::from_suffix(parts_rev[1]) {
            if let Some(caps) = SUFFIX_FIRST_PART_RE.captures(parts_rev[0]) {
                let mut parts_orig: Vec<String> = fname.to_string_lossy()
                    .split(".")
                    .map(|s| s.to_string())
                    .collect();
                let num_base_parts = parts_orig.len() - 2;
                let basename = parts_orig[..num_base_parts].iter()
                    .map(|s| &**s)
                    .collect::<Vec<_>>()
                    .join(".");

                let mut archive_parts = vec![];
                let padding = caps[1].len() + 1;
                let unicode_name = parts_rev.join(".");
                if fname != &unicode_name[..] {
                    return None;
                }
                let mut ctr: usize = 1;
                while path.exists() {
                    archive_parts.push(path.clone());
                    ctr += 1;
                    parts_orig[0] = format!("{:0padding$}", ctr, padding = padding);
                    path.set_file_name(parts_orig.join("."));
                }

                Archive::create(basename.into(), archive_parts, kind.clone())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
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