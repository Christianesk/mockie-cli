use clap::Parser;

pub mod commands;

#[derive(Parser)]
#[command(
    name = "mockie",
    version,
    about = "🎭 Configurable Mock HTTP server from CLI"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: commands::Commands,
}
