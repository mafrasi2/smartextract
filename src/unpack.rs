use anyhow::Result;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use crate::archives::{Archive, ArchiveKind};
use crate::passwords::PasswordDatabase;
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
    pub volumes: Option<Vec<PathBuf>>,
    pub files: Option<Vec<PathBuf>>,
    pub password: Option<String>,
}

fn try_unpack_7z(archive: &Archive, pdb: &PasswordDatabase) -> Result<Unpack, UnpackError> {
    let parent = archive.parts[0].parent()
        .ok_or_else(|| UnpackError::Encoding)?;
    let tmpdir = rooted_tempdir::create_rooted_tempdir(parent.into(), "TODO");
    Err(UnpackError::Unknown)
}

fn try_unpack_rar(archive: &Archive, pdb: &PasswordDatabase) -> Result<Unpack, UnpackError> {
    Err(UnpackError::Unknown)
}

pub fn try_unpack(archive: &Archive, pdb: &PasswordDatabase) -> Result<Unpack, UnpackError> {
    match archive.kind {
        ArchiveKind::P7Z => try_unpack_7z(archive, pdb),
        ArchiveKind::RAR => try_unpack_rar(archive, pdb),
    }
}