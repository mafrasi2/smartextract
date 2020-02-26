use std::path::PathBuf;
use crate::archives::Archive;
use crate::passwords::PasswordDatabase;

pub enum UnpackError {
    NoPassword,
    Incomplete,
    Unknown,
}

pub struct Unpack {
    pub volumes: Option<Vec<PathBuf>>,
    pub files: Option<Vec<PathBuf>>,
    pub password: Option<String>,
}

fn try_unpack_7z(path: &PathBuf, pdb: &PasswordDatabase) -> Result<Unpack, UnpackError> {
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