use clap::Parser;
use todo::{cli::CLI, config::Config, handler};

fn main() {
    let cli = CLI::parse();

    let config = Config::new(cli);

    let result = handler(config);

    print!("{result}");
}
