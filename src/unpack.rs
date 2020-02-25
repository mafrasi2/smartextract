use std::path::PathBuf;
use crate::archives::Archive;

pub enum UnpackError {
    NoPassword,
    Incomplete,
    Unknown,
}

pub fn try_unpack(archive: &Archive) -> Result<(), UnpackError> {
    let path = archive.path();
    println!("{}", path.as_os_str().to_string_lossy());
    Err(UnpackError::Unknown)
}