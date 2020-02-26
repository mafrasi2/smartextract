use std::collections::HashSet;

pub struct PasswordDatabase {
    pub passwords: Vec<String>
}

fn dedup_passwords(passwords: &mut Vec<String>) {
    let mut pwd_set: HashSet<String> = passwords.iter().cloned().collect();
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
    pub fn create(mut passwords: Vec<String>) -> Self {
        dedup_passwords(&mut passwords);
        Self {passwords}
    }

    pub fn promote(&mut self, correct_pwd: &str) {
        let old_position = self.passwords.iter().position(|pwd| pwd == correct_pwd);
        let old_position = match old_position {
            Some(index) => index,
            None => return
        };
        let correct_pwd = self.passwords.remove(old_position);
        self.passwords.push(correct_pwd);
    }
}