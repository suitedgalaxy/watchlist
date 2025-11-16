use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Config {
    #[arg(short, long, default_value = "watchlist.ron")]
    pub datafile: String,

    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand)]
pub enum Mode {
    /// [alias l]
    #[command(alias = "l")]
    ListAll,
    /// [alias d]
    #[command(alias = "d")]
    ListDetails {
        name: String,
    },
    /// [alias a]
    #[command(alias = "a")]
    Append,
    /// [alias e]
    #[command(alias = "e")]
    Edit {
        name: String,
    },
    /// [alias r]
    #[command(alias = "r")]
    Remove {
        name: String,
    }
}
