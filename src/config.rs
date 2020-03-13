use std::fs::File;
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::passwords::Password;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub passwords: Vec<Password>,
}

impl Config {
    fn default_config() -> Config {
        Config {
            passwords: vec![],
        }
    }

    pub fn load<P: AsRef<Path>>(config_path: P) -> Self {
        let config = if config_path.as_ref().exists() {
            let config_file = File::open(config_path).unwrap();
            serde_json::from_reader(config_file).unwrap()
        } else {
            Config::default_config()
        };
        config
    }

    pub fn store<P: AsRef<Path>>(&self, config_path: P) {
        let config_file = File::create(config_path).unwrap();
        serde_json::to_writer(config_file, self).unwrap();
    }
}