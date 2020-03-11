use anyhow::Result;
use std::error::Error;
use std::io;
use std::fmt;
use std::path::{PathBuf, Path};
use crate::archives::Archive;
use crate::passwords::{Password, PasswordDatabase};
use crate::rooted_tempdir;

#[derive(Debug)]
pub enum UnpackError {
    NoPassword,
    Incomplete,
    Encoding,
    Unknown,
    Forwarded(Box<dyn Error>),
}

impl fmt::Display for UnpackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnpackError::NoPassword => write!(f, "no password found"),
            UnpackError::Incomplete => write!(f, "incomplete archive"),
            UnpackError::Encoding => write!(f, "invalid encoding"),
            UnpackError::Unknown => write!(f, "unknown error (FIXME)"),
            UnpackError::Forwarded(error) => write!(f, "{}", error)
        }
    }
}

impl Error for UnpackError {}

pub struct Unpack {
    pub password: Password,
}

fn move_from_tempdir<P: AsRef<Path>>(parent: P, tmpdir: P) -> io::Result<()> {
    unimplemented!("move from tempdir");
}

pub fn try_unpack(archive: &Archive, pdb: &PasswordDatabase, overwrite: bool, always_dir: bool) -> Result<Unpack, UnpackError> {
    let parent = archive.parts[0].parent()
        .ok_or_else(|| UnpackError::Encoding)?;
    let mut tmpdir = rooted_tempdir::create_rooted_tempdir(
        parent.into(),
        &archive.basename.to_string_lossy()
    ).or_else(|e| Err(UnpackError::Forwarded(e.into())))?;
    let unpack = archive.unpack(&tmpdir.path, pdb, overwrite)?;
    if always_dir {
        tmpdir.keep();
    } else {
        todo!("check if there is more than one file");
        let _ = move_from_tempdir(&parent, &&*tmpdir.path);
    }
    Ok(unpack)
}