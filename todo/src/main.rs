use clap::Parser;
use todo::{cli::CLI, config::Config, handler};

fn main() {
    let cli = CLI::parse();

    let config = Config::new(cli).unwrap();

    config.prepare_database().unwrap();

    let result = handler(config).unwrap();

    print!("{result}");
}
