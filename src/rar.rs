use std::io;
use std::path::Path;
use std::process::Command;

use crate::archives::Archive;
use crate::passwords::{Password, PasswordDatabase};
use crate::temp_extract::{Extract, ExtractError};

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

static RAR_BADPWD: i32 = 11;

fn find_pwd_by_list<'a>(archive: &Archive, pdb: &'a PasswordDatabase) -> io::Result<Option<&'a Password>> {
    for pwd in &pdb.passwords {
        let mut cmd = Command::new("unrar");
        cmd.arg("l");
        encode_pwd(&mut cmd, pwd);
        cmd.arg(&archive.parts[0]);

        let status = cmd.status()?;
        if status.success() {
            return Ok(Some(pwd))
        }
        match status.code() {
            None => return Ok(None),
            Some(code) if code == RAR_BADPWD => {},
            Some(_) => return Ok(None)
        }
    }
    Ok(None)
}

fn try_extract_rar<'a, P: AsRef<Path>>(archive: &Archive, to: P, pwd: &'a Password, overwrite: bool) -> io::Result<bool> {
    let mut cmd = Command::new("unrar");
    cmd.arg("x");
    cmd.arg(if overwrite { "-o+" } else {"-o-" });
    encode_pwd(&mut cmd, pwd);
    cmd.arg(&archive.parts[0]);
    cmd.arg(to.as_ref());
    dbg!(&cmd);

    return Ok(cmd.status()?.success());
}

pub fn extract_rar<P: AsRef<Path>>(archive: &Archive, to: P, pdb: &PasswordDatabase, overwrite: bool) -> Result<Extract, ExtractError> {
    let list_res = find_pwd_by_list(archive, pdb)
        .map_err(|e| ExtractError::Forwarded(e.into()))?;
    if let Some(pwd) = list_res {
        let extract_res = try_extract_rar(archive, &to, pwd, overwrite);
        if let Ok(true) = extract_res {
            return Ok(Extract {
                password: pwd.clone()
            })
        }
    }
    // the list strategy may have detected an incorrect password, so we need to to the expensive strategy by
    // actually trying the passwords for the extraction
    for pwd in &pdb.passwords {
        if let Some(list_pwd) = list_res {
            if pwd == list_pwd {
                continue;
            }
        }
        let extract_res = try_extract_rar(archive, &to, pwd, overwrite);
        if let Ok(true) = extract_res {
            return Ok(Extract {
                password: pwd.clone()
            })
        }
    }
    Err(ExtractError::NoPassword)
}