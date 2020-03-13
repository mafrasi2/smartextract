use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::process::{Command, Output};

use crate::archives::Archive;
use crate::passwords::{Password, PasswordDatabase};
use crate::temp_extract::{Extract, ExtractError};

enum P7ZResult<'a> {
    Success(&'a Password),
    NoPasswordFound,
    Corrupt,
}

fn parse_7z_output<'a>(output: &Output, pwd: &'a Password) -> P7ZResult<'a> {
    if output.status.success() {
        return P7ZResult::Success(pwd);
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.contains("Wrong password?") {
        P7ZResult::NoPasswordFound
    } else {
        P7ZResult::Corrupt
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

fn find_pwd_by_list<'a>(archive: &Archive, pdb: &'a PasswordDatabase) -> io::Result<P7ZResult<'a>> {
    for pwd in &pdb.passwords {
        let mut cmd = Command::new("7z");
        cmd.arg("l");
        encode_pwd(&mut cmd, pwd);
        cmd.arg(&archive.parts[0]);

        let result = parse_7z_output(&cmd.output()?, pwd);
        if let P7ZResult::NoPasswordFound = result {
            continue;
        }
        return Ok(result);
    }
    Ok(P7ZResult::NoPasswordFound)
}

fn try_extract_7z<'a, P: AsRef<Path>>(archive: &Archive, to: P, pwd: &'a Password, overwrite: bool) -> io::Result<P7ZResult<'a>> {
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
    let list_res = find_pwd_by_list(archive, pdb)
        .map_err(|e| ExtractError::Forwarded(e.into()))?;
    match list_res {
        P7ZResult::NoPasswordFound => return Err(ExtractError::NoPassword),
        P7ZResult::Corrupt => return Err(ExtractError::Incomplete),
        P7ZResult::Success(pwd) => {
            let extract_res = try_extract_7z(archive, &to, pwd, overwrite);
            if let Ok(P7ZResult::Success(pwd)) = extract_res {
                return Ok(Extract {
                    password: pwd.clone()
                })
            };
        },
    }
    // the list strategy may have detected an incorrect password, so we need to to the expensive strategy by
    // actually trying the passwords for the extraction
    for pwd in &pdb.passwords {
        if let P7ZResult::Success(list_pwd) = list_res {
            if pwd == list_pwd {
                continue;
            }
        }
        let extract_res = try_extract_7z(archive, &to, pwd, overwrite);
        if let Ok(P7ZResult::Success(pwd)) = extract_res {
            return Ok(Extract {
                password: pwd.clone()
            })
        };
    }
    Err(ExtractError::NoPassword)
}