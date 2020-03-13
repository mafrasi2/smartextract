use std::error::Error;
use std::fmt;
use std::io;
use std::path::Path;

use crate::passwords::{Password, PasswordAttempt, PasswordDatabase};

pub type TryPwdByList<'a> = fn(first_part: &Path, pwd: &'a Password) -> io::Result<PasswordAttempt<'a>>;
pub type TryExtract<'a> = fn(first_part: &Path, pwd: &'a Password, to: &Path) -> io::Result<PasswordAttempt<'a>>;

#[derive(Debug)]
pub enum ExtractError {
    NoPassword,
    Incomplete,
    Encoding,
    Forwarded(Box<dyn Error>),
}

impl fmt::Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExtractError::NoPassword => write!(f, "no password found"),
            ExtractError::Incomplete => write!(f, "incomplete archive"),
            ExtractError::Encoding => write!(f, "invalid encoding"),
            ExtractError::Forwarded(error) => write!(f, "{}", error)
        }
    }
}

impl Error for ExtractError {}

pub struct Extract {
    pub password: Password,
}

pub fn extract<'a>(
        first_part: &Path,
        to: &Path,
        pdb: &'a PasswordDatabase,
        try_pwd_by_list: TryPwdByList<'a>,
        try_extract: TryExtract<'a>
) -> Result<Extract, ExtractError> {
    let mut found_pwd = None;
    for pwd in &pdb.passwords {
        let list_res = try_pwd_by_list(first_part, pwd);
        match list_res {
            Err(e) => return Err(ExtractError::Forwarded(e.into())),
            Ok(PasswordAttempt::CorruptArchive) => return Err(ExtractError::Incomplete),
            Ok(PasswordAttempt::Correct(pwd)) => {
                let extract_res = try_extract(first_part, pwd, to);
                match extract_res {
                    Ok(PasswordAttempt::Correct(pwd)) => return Ok(Extract {
                        password: pwd.clone()
                    }),
                    Ok(PasswordAttempt::Incorrect) => {
                        // the list strategy may have returned the wrong password
                        found_pwd = Some(pwd);
                        break;
                    },
                    _ => return Err(ExtractError::Incomplete)
                }
            },
            Ok(PasswordAttempt::Incorrect) => {}
        }
    }

    for pwd in &pdb.passwords {
        if let Some(found_pwd) = found_pwd {
            if pwd == found_pwd {
                continue;
            }
        }
        let extract_res = try_extract(first_part, pwd, to);
        match extract_res {
            Err(e) => return Err(ExtractError::Forwarded(e.into())),
            Ok(PasswordAttempt::CorruptArchive) => return Err(ExtractError::Incomplete),
            Ok(PasswordAttempt::Correct(pwd)) => {
                return Ok(Extract {
                    password: pwd.clone()
                })
            }
            Ok(PasswordAttempt::Incorrect) => {},
        }
    }
    Err(ExtractError::NoPassword)
}