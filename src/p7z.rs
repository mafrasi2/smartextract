use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::process::{Command, Output};

use crate::archives::Archive;
use crate::passwords::{Password, PasswordDatabase, PasswordAttempt};
use crate::temp_extract::{Extract, ExtractError};

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

fn try_pwd_by_list<'a>(archive: &Archive, pwd: &'a Password) -> io::Result<PasswordAttempt<'a>> {
    let mut cmd = Command::new("7z");
    cmd.arg("l");
    encode_pwd(&mut cmd, pwd);
    cmd.arg(&archive.parts[0]);

    Ok(parse_7z_output(&cmd.output()?, pwd))
}

fn try_extract_7z<'a, P: AsRef<Path>>(archive: &Archive, pwd: &'a Password, to: P, overwrite: bool) -> io::Result<PasswordAttempt<'a>> {
    let mut cmd = Command::new("7z");
    let mut output_arg: OsString = "-o".into();
    output_arg.push(to.as_ref());
    cmd.arg("x")
       .arg(output_arg);
    cmd.arg(if overwrite { "-aoa" } else {"-aos" });
    encode_pwd(&mut cmd, pwd);
    cmd.arg(&archive.parts[0]);

    return Ok(parse_7z_output(&cmd.output()?, pwd));
}


pub fn extract_7z<P: AsRef<Path>>(archive: &Archive, to: P, pdb: &PasswordDatabase, overwrite: bool) -> Result<Extract, ExtractError> {
    let mut found_pwd = None;
    for pwd in &pdb.passwords {
        let list_res = try_pwd_by_list(archive, pwd);
        match list_res {
            Err(e) => return Err(ExtractError::Forwarded(e.into())),
            Ok(PasswordAttempt::CorruptArchive) => return Err(ExtractError::Incomplete),
            Ok(PasswordAttempt::Correct(pwd)) => {
                let extract_res = try_extract_7z(archive, pwd, &to, overwrite);
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
        let extract_res = try_extract_7z(archive, pwd, &to, overwrite);
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