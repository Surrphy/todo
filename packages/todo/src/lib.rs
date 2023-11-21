use anyhow::Result;
use config::Config;
use cli::Commands::{Add, Do, Undo, Delete, List};
use handlers::{add, list, change_done, delete};

pub mod cli;
pub mod config;
pub mod handlers;

/// Get cli config and run specified subcomands
///
/// # Errors: Return error from handlers (`PersyError`)
pub fn handler(config: Config) -> Result<String> {
    match config.cli.command {
        Add { desc, done } => add(&config.db, &desc, done),
        Do { index } => change_done(&config.db, &index, true),
        Undo { index } => change_done(&config.db, &index, false),
        Delete { index } => delete(&config.db, &index),
        List => list(&config.db)
    }
}
