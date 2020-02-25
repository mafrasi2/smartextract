use std::env;

mod config;

fn main() {
    let cfg = config::Config::load();
    for arg in env::args() {
        
    }
    cfg.store();
}