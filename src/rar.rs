use std::io;
use std::path::Path;
use std::process::Command;

use crate::archives::Archive;
use crate::passwords::{Password, PasswordDatabase, PasswordAttempt};
use crate::temp_extract::{Extract, ExtractError};

static RAR_BADPWD: i32 = 11;

fn encode_pwd(cmd: &mut Command, pwd: &Password) {
    match pwd {
        Password::NoPassword => {
            cmd.arg("-p-")
        },
        Password::Password(pwd) => {
            cmd.arg(format!("-p{}", pwd))
        }
    };
}

fn try_pwd_by_list<'a>(archive: &Archive, pwd: &'a Password) -> io::Result<PasswordAttempt<'a>> {
    let mut cmd = Command::new("unrar");
    cmd.arg("l");
    encode_pwd(&mut cmd, pwd);
    cmd.arg(&archive.parts[0]);
    dbg!(&cmd);

    let status = cmd.status()?;
    if status.success() {
        Ok(PasswordAttempt::Correct(pwd))
    } else {
        match status.code() {
            Some(code) if code == RAR_BADPWD => Ok(PasswordAttempt::Incorrect),
            _ => Ok(PasswordAttempt::CorruptArchive)
        }
    }
}

fn try_extract_rar<'a, P: AsRef<Path>>(archive: &Archive, pwd: &'a Password, to: P, overwrite: bool) -> io::Result<PasswordAttempt<'a>> {
    let mut cmd = Command::new("unrar");
    if let Some(parent) = archive.parts[0].parent() {
        cmd.current_dir(parent);
    }
    cmd.arg("x");
    cmd.arg(if overwrite { "-o+" } else {"-o-" });
    encode_pwd(&mut cmd, pwd);
    cmd.arg(&archive.parts[0]);
    cmd.arg(to.as_ref());
    dbg!(&cmd);

    let status = cmd.status()?;
    if status.success() {
        Ok(PasswordAttempt::Correct(pwd))
    } else {
        match status.code() {
            Some(code) if code == RAR_BADPWD => Ok(PasswordAttempt::Incorrect),
            _ => Ok(PasswordAttempt::CorruptArchive),
        }
    }
}

pub fn extract_rar<P: AsRef<Path>>(archive: &Archive, to: P, pdb: &PasswordDatabase, overwrite: bool) -> Result<Extract, ExtractError> {
    let mut found_pwd = None;
    for pwd in &pdb.passwords {
        let list_res = try_pwd_by_list(archive, pwd);
        match list_res {
            Err(e) => return Err(ExtractError::Forwarded(e.into())),
            Ok(PasswordAttempt::CorruptArchive) => return Err(ExtractError::Incomplete),
            Ok(PasswordAttempt::Correct(pwd)) => {
                let extract_res = try_extract_rar(archive, pwd, &to, overwrite);
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
        let extract_res = try_extract_rar(archive, pwd, &to, overwrite);
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