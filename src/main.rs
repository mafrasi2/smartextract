use std::env;
use std::fs;
use std::path::PathBuf;

mod archives;
mod config;
mod unpack;

fn do_archives(archive: &archives::Archive) {
    let path = archive.path();
    print!("{}...", path.as_os_str().to_string_lossy());
    match unpack::try_unpack(archive) {
        Err(unpack::UnpackError::NoPassword) => {print!("no password")},
        Err(unpack::UnpackError::Incomplete) => {print!{"incomplete archive"}},
        Err(unpack::UnpackError::Unknown) => {print!{"unkown error"}},
        Ok(_) => {
            archives::delete_archive(path);
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
            let archive = match archives::detect_archive(path.clone()) {
                Some(archive) => archive,
                None => {
                    println!("not a supported archive: {}", path.as_os_str().to_string_lossy(),);
                    continue;
                }
            };
            do_archives(&archive);
        } else {
            let archives = match fs::read_dir(&path) {
                Ok(archives) => archives,
                Err(err) => {
                    println!("can't iterate {}: {}", path.as_os_str().to_string_lossy(), err);
                    continue;
                }
            };
            for entry in archives {
                let entry = match entry {
                    Ok(entry) => entry.path(),
                    Err(err) => {
                        println!("error while iterating {}: {}", path.as_os_str().to_string_lossy(), err);
                        break;
                    }
                };
                match archives::detect_archive(entry) {
                    Some(archive) => do_archives(&archive),
                    None => {}
                }
            }
        }
    }
    cfg.store();
}