use clap::Clap;
use std::fs;
use std::path::PathBuf;
use xdg::BaseDirectories;

mod archives;
mod config;
mod extract;
mod passwords;
mod rooted_tempdir;
mod temp_extract;
mod rar;
mod p7z;

fn do_archive(archive: &archives::Archive, pdb: &mut passwords::PasswordDatabase, overwrite: bool, always_dirs: bool) {
    print!("{}...", archive.parts[0].as_os_str().to_string_lossy());
    match temp_extract::try_extract(archive, pdb, overwrite, always_dirs) {
        Err(e) => {print!{"{}", e}},
        Ok(result) => {
            print!("success");
            match archive.delete() {
                Ok(_) => {},
                Err(err) => print!(" ({})", err)
            };
            pdb.promote(&result.password);
        }
    };
    println!();
}

#[derive(Clap)]
#[clap(version = "1.0", author = "Max Sistemich")]
struct Opts {
    /// Either files to extract or directories that contain files
    inputs: Vec<PathBuf>,
    /// Overwrite existing files (currently unused)
    #[clap(short, long)]
    overwrite: bool,
    /// Always create directories
    #[clap(short, long)]
    directories: bool,
    /// Path to the config file, defaults to ~/.config/smartextract.json
    #[clap(short, long)]
    config: Option<PathBuf>
}

fn main() {
    let opts: Opts = Opts::parse();

    let cfg_path = match opts.config {
        Some(cfg_path) => cfg_path,
        None => {
            let base_dirs = BaseDirectories::new().unwrap();
            base_dirs.place_config_file("smartextract.json").unwrap()
        }
    };
    let mut cfg = config::Config::load(&cfg_path);

    let paths = if opts.inputs.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        opts.inputs
    };

    let mut pdb = passwords::PasswordDatabase::create(cfg.passwords.clone());

    for path in paths {
        if path.is_file() {
            let archive = match archives::detect_archive(path.clone()) {
                Some(archive) => archive,
                None => {
                    println!("not a supported archive: {}", path.as_os_str().to_string_lossy(),);
                    continue;
                }
            };
            do_archive(&archive, &mut pdb, opts.overwrite, opts.directories);
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
                    Some(archive) => do_archive(&archive, &mut pdb, opts.overwrite, opts.directories),
                    None => {}
                }
            }
        }
    }
    cfg.passwords = pdb.passwords;
    cfg.store(&cfg_path);
}