use anyhow::Result;
use std::error::Error;
use std::io;
use std::fmt;
use std::fs;
use std::path::Path;
use crate::archives::Archive;
use crate::passwords::{Password, PasswordDatabase};
use crate::rooted_tempdir;

#[derive(Debug)]
pub enum ExtractError {
    NoPassword,
    Incomplete,
    Encoding,
    Forwarded(Box<dyn Error>),
}

impl fmt::Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExtractError::NoPassword => write!(f, "no password found"),
            ExtractError::Incomplete => write!(f, "incomplete archive"),
            ExtractError::Encoding => write!(f, "invalid encoding"),
            ExtractError::Forwarded(error) => write!(f, "{}", error)
        }
    }
}

impl Error for ExtractError {}

pub struct Extract {
    pub password: Password,
}

fn try_move_from_tempdir<P1: AsRef<Path>, P2: AsRef<Path>>(tmpdir: P1, to: P2) -> io::Result<bool> {
    let mut dest = to.as_ref().to_path_buf();
    for entry in fs::read_dir(&tmpdir)? {
        let src = entry?.path();
        let fname = match src.file_name() {
            None => return Ok(false),
            Some(fname) => fname
        };
        dest.push(fname);
        if dest.exists() {
            return Ok(false)
        }
        dest.pop();
    }

    for entry in fs::read_dir(&tmpdir)? {
        let src = entry?.path();
        let fname = match src.file_name() {
            None => return Ok(false),
            Some(fname) => fname
        };
        dest.push(fname);
        fs::rename(src, &dest)?;
        dest.pop();
    }
    Ok(true)
}

pub fn try_extract(archive: &Archive, pdb: &PasswordDatabase, overwrite: bool, always_dir: bool) -> Result<Extract, ExtractError> {
    let parent = archive.parts[0].parent()
        .ok_or_else(|| ExtractError::Encoding)?;
    let mut tmpdir = rooted_tempdir::create_rooted_tempdir(
        parent.into(),
        &archive.basename.to_string_lossy()
    ).or_else(|e| Err(ExtractError::Forwarded(e.into())))?;
    let extract = archive.extract(&tmpdir.path, pdb, overwrite)?;
    if always_dir {
        tmpdir.keep();
    } else {
        let mut rdir = fs::read_dir(&tmpdir.path)
            .map_err(|e| ExtractError::Forwarded(e.into()))?;
        rdir.next();
        if let None = rdir.next() {
            // only move empty or one-element results
            let res = try_move_from_tempdir(&tmpdir.path, &parent);
            if let Ok(false) = res {
                tmpdir.keep();
            }
        } else {
            tmpdir.keep();
        }
    }
    Ok(extract)
}