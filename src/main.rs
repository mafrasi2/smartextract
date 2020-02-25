use std::env;
use std::fs;
use std::path::PathBuf;

mod config;
mod unpack;

fn do_unpack(archive: &PathBuf) {
    print!("{}...", archive.as_os_str().to_string_lossy());
    match unpack::try_unpack(&archive) {
        Err(unpack::UnpackError::NoPassword) => {print!("no password")},
        Err(unpack::UnpackError::Incomplete) => {print!{"incomplete archive"}},
        Err(unpack::UnpackError::Unknown) => {print!{"unkown error"}},
        Ok(_) => {
            unpack::delete_archive(&archive);
            print!("success")
        }
    };
    println!();
}

fn main() {
    let cfg = config::Config::load();
    let args = env::args();
    let paths = if args.len() > 1 {
        args.into_iter()
            .skip(1)
            .map(|s| PathBuf::from(s))
            .collect()
    } else {
        vec![PathBuf::from(".")]
    };

    for path in paths {
        if path.is_file() {
            do_unpack(&path);
        } else {
            let archives = match fs::read_dir(&path) {
                Ok(archives) => archives,
                Err(err) => {
                    println!("error iterating {}: {}", path.as_os_str().to_string_lossy(), err);
                    continue;
                }
            };
            for entry in archives {
                let entry = match entry {
                    Ok(entry) => entry.path(),
                    Err(err) => {
                        println!("error iterating {}: {}", path.as_os_str().to_string_lossy(), err);
                        break;
                    }
                };
                if !unpack::is_archive(&entry) {
                    continue;
                }
                do_unpack(&entry);
            }
        }
    }
    cfg.store();
}