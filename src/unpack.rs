use anyhow::Result;
use std::error::Error;
use std::io;
use std::fmt;
use std::path::{PathBuf, Path};
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

fn try_unpack_7z<P: AsRef<Path>>(archive: &Archive, to: P, pdb: &PasswordDatabase, overwrite: bool, always_dir: bool) -> Result<Unpack, UnpackError> {
    unimplemented!("7z extraction");
}

fn try_unpack_rar<P: AsRef<Path>>(archive: &Archive, to: P, pdb: &PasswordDatabase, overwrite: bool, always_dir: bool) -> Result<Unpack, UnpackError> {
    unimplemented!("rar extraction");
}

fn move_from_tempdir<P: AsRef<Path>>(parent: P, tmpdir: P) -> io::Result<()> {
    unimplemented!("move from tempdir");
}

pub fn try_unpack(archive: &Archive, pdb: &PasswordDatabase, overwrite: bool, always_dir: bool) -> Result<Unpack, UnpackError> {
    let parent = archive.parts[0].parent()
        .ok_or_else(|| UnpackError::Encoding)?;
    let tmpdir = rooted_tempdir::create_rooted_tempdir(
        parent.into(),
        &archive.basename.to_string_lossy()
    ).or_else(|e| Err(UnpackError::Forwarded(e.into())))?;
    let unpack_res = match archive.kind {
        ArchiveKind::P7Z => try_unpack_7z(archive, &tmpdir.path, pdb, overwrite, always_dir),
        ArchiveKind::RAR => try_unpack_rar(archive, &tmpdir.path, pdb, overwrite, always_dir),
    };
    let unpack = match unpack_res {
        Err(_) => return unpack_res,
        Ok(unpack) => unpack
    };
    let _ = move_from_tempdir(&parent, &&*tmpdir.path);
    Ok(unpack)
}