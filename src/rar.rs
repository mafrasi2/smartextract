use std::io;
use std::path::Path;
use std::process::Command;

use crate::passwords::{Password, PasswordAttempt};

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

pub fn try_pwd_by_list<'a>(first_part: &Path, pwd: &'a Password) -> io::Result<PasswordAttempt<'a>> {
    let mut cmd = Command::new("unrar");
    cmd.arg("l");
    encode_pwd(&mut cmd, pwd);
    cmd.arg(first_part);

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

pub fn try_extract<'a>(first_part: &Path, pwd: &'a Password, to: &Path) -> io::Result<PasswordAttempt<'a>> {
    let mut cmd = Command::new("unrar");
    if let Some(parent) = first_part.parent() {
        cmd.current_dir(parent);
    }
    cmd.arg("x");
    cmd.arg("-o+");
    encode_pwd(&mut cmd, pwd);
    cmd.arg(first_part);
    cmd.arg(to);

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
