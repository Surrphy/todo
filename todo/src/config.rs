use std::{env, path::PathBuf};

use anyhow::{anyhow, Result};

use persy::{Persy, Config as PersyConfig, CreateError, PE::PE, CreateSegmentError};

use crate::cli::CLI;

pub struct Config {
    pub db: Persy,
    pub username: String,
    pub cli: CLI
}

impl Config {
    pub fn new(cli: CLI) -> Result<Config>{
        let username = env::var("USER").unwrap();
        let persy_path = PathBuf::from(format!("{}/todo.db", env::var("TODO_CONFIG").unwrap()));

        if let Err(PE(CreateError::Generic(err))) = Persy::create(&persy_path) {
            return Err(anyhow!(err).context(format!("Error while creating persy db at {persy_path:?}")))
        }

        let persy_db = Persy::open(persy_path, PersyConfig::new()).unwrap();

        Ok(
            Config {
                db: persy_db,
                username,
                cli
            }
        )
    }

    pub fn prepare_database(&self) -> Result<()> {
        let mut tx = self.db.begin()?;

        if let Err(PE(CreateSegmentError::Generic(err))) = tx.create_segment("todos") {
            return Err(anyhow!(err).context("Error while creating todos segment"))
        }

        let prepared = tx.prepare()?;

        prepared.commit()?;

        Ok(())
    }
}
