use clap::Parser;

pub mod commands;

#[derive(Parser)]
#[command(
    name = "mockie",
    version,
    about = "🎭 Mock HTTP server configurable por CLI"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: commands::Commands,
}
