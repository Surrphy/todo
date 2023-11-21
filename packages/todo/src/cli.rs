use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Add {        
        desc: String,

        #[arg(short, long)]
        done: bool
    },

    Do {
        index: String
    },

    Undo {
        index: String
    },

    Delete {
        index: String
    },

    List
}
