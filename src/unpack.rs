use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use crate::archives::Archive;
use crate::passwords::PasswordDatabase;
use crate::rooted_tempdir;

#[derive(Debug)]
pub enum UnpackError {
    NoPassword,
    Incomplete,
    Unknown,
    Forwarded(Box<dyn Error>),
}

impl fmt::Display for UnpackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NoPassword => write!(f, "no password found"),
            Incomplete => write!(f, "incomplete archive"),
            Unknown => write!(f, "unknown error (FIXME)"),
            UnpackError::Forwarded(error) => write!(f, "{}", error)
        }
    }
}

impl<'a> Error for UnpackError {
    fn source(&'a self) -> Option<&'a (dyn Error + 'static)> {
        match self {
            UnpackError::Forwarded(error) => error,
            _ => None
        }
    }
}

pub struct Unpack {
    pub volumes: Option<Vec<PathBuf>>,
    pub files: Option<Vec<PathBuf>>,
    pub password: Option<String>,
}

fn try_unpack_7z(path: &PathBuf, pdb: &PasswordDatabase) -> Result<Unpack, UnpackError> {
    let tmpdir = rooted_tempdir::create_rooted_tempdir(path.parent(), "Hello");
    let parent = path.parent().expect("file has no parent directory");
    Err(UnpackError::Unknown)
}

fn try_unpack_rar(path: &PathBuf, pdb: &PasswordDatabase) -> Result<Unpack, UnpackError> {
    Err(UnpackError::Unknown)
}

pub fn try_unpack(archive: &Archive, pdb: &PasswordDatabase) -> Result<Unpack, UnpackError> {
    match archive {
        Archive::P7Z(path) => try_unpack_7z(path, pdb),
        Archive::RAR(path) => try_unpack_rar(path, pdb),
    }
}