use std::fs;
use std::io::Result;
use std::path::PathBuf;

pub struct RootedTempDir {
    pub path: PathBuf
}

pub fn create_rooted_tempdir(parent: PathBuf, name: &str) -> Result<RootedTempDir> {
    let mut child = parent;
    child.push(name);
    if child.exists() {
        // worst case quadratic runtime, but I don't care right now
        for i in 1.. {
            child.set_file_name(format!("{}{}", name, i));
            if !child.exists() {
                break;
            }
        }
    }
    fs::create_dir(&child)?;
    Ok(RootedTempDir {
        path: child
    })
}

impl Drop for RootedTempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}