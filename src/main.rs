use std::env;
use std::fs::File;
use std::path::PathBuf;
use xdg::BaseDirectories;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    passwords: Vec<String>,
}

fn default_config() -> Config {
    Config {
        passwords: vec![],
    }
}

fn get_config_file() -> PathBuf {
    let base_dirs = BaseDirectories::new().unwrap();
    base_dirs.place_config_file("smartunpack.json").unwrap()
}

fn load_config() -> Config {
    let config_path = get_config_file();
    let config = if config_path.exists() {
        let config_file = File::open(config_path).unwrap();
        serde_json::from_reader(config_file).unwrap()
    } else {
        default_config()
    };
    config
}

fn store_config(config: &Config) {
    let config_path = get_config_file();
    let config_file = File::create(config_path).unwrap();
    serde_json::to_writer(config_file, config).unwrap();
}

fn main() {
    let cfg = load_config();
    for arg in env::args() {
        
    }
    store_config(&cfg);
}
