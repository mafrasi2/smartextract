use std::path::Path;

use crate::archives::Archive;
use crate::passwords::PasswordDatabase;
use crate::temp_extract::{Extract, ExtractError};

pub fn extract_rar<P: AsRef<Path>>(_archive: &Archive, _to: P, _pdb: &PasswordDatabase, _overwrite: bool) -> Result<Extract, ExtractError> {
    unimplemented!("rar extraction");
}