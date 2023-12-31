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
    /// Generate new config from cli information
    ///
    /// # Errors
    ///
    /// Errors if:
    ///   - config path can't be initialized
    ///   - USER env var isn't set
    ///   - Persy encounters a generic error while creating db
    ///   - Persy can't open database
    pub fn new(cli: CLI) -> Result<Config> {
        let Ok(username) = env::var("USER") else {
            return Err(anyhow!("USER env var not set"))
        };

        let (_config_path, persy_path) = get_config_path()?;

        if let Err(PE(CreateError::Generic(err))) = Persy::create(&persy_path) {
            return Err(anyhow!(err).context(format!("Error while creating persy db at {persy_path:?}")))
        }

        let persy_db = Persy::open(persy_path, PersyConfig::new())?;

        Ok(
            Config {
                db: persy_db,
                username,
                cli
            }
        )
    }

    /// Create all required segments if they don't already exist
    ///
    /// # Errors
    ///
    /// Errors from begining and commiting transactions
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

fn get_config_path() -> Result<(PathBuf, PathBuf)> {
    let config_path = match env::var("TODO_CONFIG") {
        Ok(v) => v,
        Err(env::VarError::NotPresent) => format!("{}/.config/todo", env::var("HOME").unwrap()),
        Err(err) => return Err(anyhow!(err).context("TODO_CONFIG env var is not in unicode"))
    };

    Ok((PathBuf::from(config_path.clone()), PathBuf::from(format!("{config_path}/todo.db"))))
}
