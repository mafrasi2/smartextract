use std::path::Path;
use std::process::Command;

use crate::archives::Archive;
use crate::passwords::PasswordDatabase;
use crate::temp_unpack::{Unpack, UnpackError};

pub fn unpack_7z<P: AsRef<Path>>(archive: &Archive, to: P, pdb: &PasswordDatabase, overwrite: bool) -> Result<Unpack, UnpackError> {
    let cmd = Command::new("7z")
        .arg("x")
        .arg("-o")
        .arg(to.as_ref())
        .arg(&archive.parts[0])
        .spawn();
    let child = match cmd {
        Ok(child) => child,
        Err(e) => return Err(UnpackError::Forwarded(e.into()))
    };
    Ok(Unpack {
        volumes: None,
        files: None,
        password: None
    })
}