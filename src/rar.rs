use std::path::Path;

use crate::archives::Archive;
use crate::passwords::PasswordDatabase;
use crate::temp_unpack::{Unpack, UnpackError};

pub fn unpack_rar<P: AsRef<Path>>(archive: &Archive, to: P, pdb: &PasswordDatabase, overwrite: bool) -> Result<Unpack, UnpackError> {
    unimplemented!("rar extraction");
}