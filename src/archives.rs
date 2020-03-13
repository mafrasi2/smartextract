use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use lazy_static;
use regex::Regex;

use crate::passwords::PasswordDatabase;
use crate::extract::{Extract, ExtractError, TryExtract, TryPwdByList};
use crate::rar;
use crate::p7z;
use crate::extract::extract;

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

    pub fn extract<'a, P: AsRef<Path>>(&self, to: P, pdb: &'a PasswordDatabase) -> Result<Extract, ExtractError> {
        let (try_pwd_by_list, try_extract): (TryPwdByList, TryExtract) = match self.kind {
            ArchiveKind::P7Z => (p7z::try_pwd_by_list, p7z::try_extract),
            ArchiveKind::RAR => (rar::try_pwd_by_list, rar::try_extract)
        };
        extract(&self.parts[0], to.as_ref(), pdb, try_pwd_by_list, try_extract)
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

    let parts_rev: Vec<_> = fname_lc.rsplit(".").collect();
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
            let unicode_name = parts_orig.join(".");
            if fname != &unicode_name[..] {
                return None;
            }
            let mut ctr: usize = 1;
            while path.exists() {
                archive_parts.push(path.clone());
                ctr += 1;
                let parts_len = parts_orig.len();
                parts_orig[parts_len - 2] = format!("part{:0padding$}", ctr, padding = padding);
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
                let unicode_name = parts_orig.join(".");
                if fname != &unicode_name[..] {
                    return None;
                }
                let mut ctr: usize = 1;
                while path.exists() {
                    archive_parts.push(path.clone());
                    ctr += 1;
                    let parts_len = parts_orig.len();
                    parts_orig[parts_len - 1] = format!("{:0padding$}", ctr, padding = padding);
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