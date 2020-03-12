use std::collections::HashSet;
use std::cmp::Eq;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Password {
    NoPassword,
    Password(String)
}

pub struct PasswordDatabase {
    pub passwords: Vec<Password>
}

fn dedup_passwords(passwords: &mut Vec<Password>) {
    let mut pwd_set: HashSet<Password> = passwords.iter().cloned().collect();
    passwords.retain(|pwd| {
        if pwd_set.contains(pwd) {
            pwd_set.remove(pwd);
            true
        } else {
            false
        }
    });
}

impl PasswordDatabase {
    pub fn create(mut passwords: Vec<Password>) -> Self {
        if passwords.len() == 0 {
            passwords.push(Password::NoPassword);
        }
        dedup_passwords(&mut passwords);
        Self {passwords}
    }

    pub fn promote(&mut self, correct_pwd: &Password) {
        let old_position = self.passwords.iter().position(|pwd| pwd == correct_pwd);
        let old_position = match old_position {
            Some(index) => index,
            None => return
        };
        let correct_pwd = self.passwords.remove(old_position);
        self.passwords.push(correct_pwd);
    }
}