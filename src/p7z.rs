use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::process::{Command, Output};

use crate::passwords::{Password, PasswordAttempt};

fn parse_7z_output<'a>(output: &Output, pwd: &'a Password) -> PasswordAttempt<'a> {
    if output.status.success() {
        PasswordAttempt::Correct(pwd)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("Wrong password?") {
            PasswordAttempt::Incorrect
        } else {
            PasswordAttempt::CorruptArchive
        }
    }
}

fn encode_pwd(cmd: &mut Command, pwd: &Password) {
    match pwd {
        Password::NoPassword => {
            cmd.arg("-p")
        },
        Password::Password(pwd) => {
            cmd.arg(format!("-p{}", pwd))
        }
    };
}

pub fn try_pwd_by_list<'a>(first_part: &Path, pwd: &'a Password) -> io::Result<PasswordAttempt<'a>> {
    let mut cmd = Command::new("7z");
    cmd.arg("l");
    encode_pwd(&mut cmd, pwd);
    cmd.arg(first_part);

    Ok(parse_7z_output(&cmd.output()?, pwd))
}

pub fn try_extract<'a>(first_part: &Path, pwd: &'a Password, to: &Path) -> io::Result<PasswordAttempt<'a>> {
    let mut cmd = Command::new("7z");
    let mut output_arg: OsString = "-o".into();
    output_arg.push(to);
    cmd.arg("x")
       .arg(output_arg);
    cmd.arg("-aoa");
    encode_pwd(&mut cmd, pwd);
    cmd.arg(first_part);

    return Ok(parse_7z_output(&cmd.output()?, pwd));
}