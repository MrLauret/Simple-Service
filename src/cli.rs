use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        ip: String,
        port: u16,
        name: String
    },

    Delete {
        name: String
    },

    GetIp {
        name: String
    },

    Update {
        action: String,
        name: String,
        new_value: String
    },

    Test {
        name: String
    },

    TestAll,

    List,
}