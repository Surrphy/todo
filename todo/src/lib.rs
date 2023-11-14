use anyhow::Result;
use config::Config;
use cli::Commands::{*};
use handlers::{add, list, change_done, delete};

pub mod cli;
pub mod config;
pub mod handlers;

pub fn handler(config: Config) -> Result<String> {

    let result = match config.cli.command {
        Add { desc, done } => add(config.db, desc, done),
        Do { index } => change_done(config.db, index, true),
        Undo { index } => change_done(config.db, index, false),
        Delete { index } => delete(config.db, index),
        List => list(config.db)
    };

    return result
}
