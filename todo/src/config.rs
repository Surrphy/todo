use std::{env, path::PathBuf};

use persy::{Persy, Config as PersyConfig};

use crate::cli::CLI;

pub struct Config {
    pub db: Persy,
    pub username: String,
    pub cli: CLI
}

impl Config {
    pub fn new(cli: CLI) -> Config{
        let username = env::var("USER").unwrap();
        let persy_path = PathBuf::from(format!("{}/.config/todo/todo.db", env::var("HOME").unwrap()));

        //Persy::create(&persy_path).unwrap();

        let persy_db = Persy::open(persy_path, PersyConfig::new()).unwrap();

        Config {
            db: persy_db,
            username,
            cli
        }
    }
}
