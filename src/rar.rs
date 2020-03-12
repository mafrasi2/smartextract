use std::path::Path;

use crate::archives::Archive;
use crate::passwords::PasswordDatabase;
use crate::temp_unpack::{Unpack, UnpackError};

pub fn unpack_rar<P: AsRef<Path>>(_archive: &Archive, _to: P, _pdb: &PasswordDatabase, _overwrite: bool) -> Result<Unpack, UnpackError> {
    unimplemented!("rar extraction");
}