use std::fs::File;
use std::path::PathBuf;
use xdg::BaseDirectories;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub passwords: Vec<String>,
}

impl Config {
    fn default_config() -> Config {
        Config {
            passwords: vec![],
        }
    }

    fn get_config_file() -> PathBuf {
        let base_dirs = BaseDirectories::new().unwrap();
        base_dirs.place_config_file("smartunpack.json").unwrap()
    }

    pub fn load() -> Self {
        let config_path = Config::get_config_file();
        let config = if config_path.exists() {
            let config_file = File::open(config_path).unwrap();
            serde_json::from_reader(config_file).unwrap()
        } else {
            Config::default_config()
        };
        config
    }

    pub fn store(&self) {
        let config_path = Config::get_config_file();
        let config_file = File::create(config_path).unwrap();
        serde_json::to_writer(config_file, self).unwrap();
    }
}