use std::fs::{File, ReadDir};
use std::path::PathBuf;
use lazy_static;
use regex::Regex;

pub enum UnpackError {
    NoPassword,
    Incomplete,
    Unknown,
}

pub fn is_archive(path: &PathBuf) -> bool {
    lazy_static::lazy_static! {
        static ref RAR_RE: Regex = Regex::new("(part0*1)?\\.rar$").unwrap();
        static ref P7Z_RE: Regex = Regex::new("7z(\\.0*1)?$").unwrap();
    }

    let fname = match path.file_name() {
        Some(fname) => fname,
        None => return false
    };
    let fname = fname.to_string_lossy();

    match RAR_RE.find(&fname) {
        Some(_) => return true,
        None => {}
    };
    match P7Z_RE.find(&fname) {
        Some(_) => return true,
        None => {}
    };
    false
}

pub fn try_unpack(path: &PathBuf) -> Result<(), UnpackError> {
    println!("{}", path.as_os_str().to_string_lossy());
    Err(UnpackError::Unknown)
}

pub fn delete_archive(path: &PathBuf) {
}