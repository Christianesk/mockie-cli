mod cli;
mod config;
mod error;
mod handlers;
mod models;
mod server;
mod storage;

use clap::Parser;
use config::Config;
use error::Result;
use serde_json::Value;
use storage::FileStorage;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let config = Config::from_env();

    match cli.command {
        cli::commands::Commands::Serve { port } => {
            // Use CLI port if provided, otherwise use config port
            let port = port.unwrap_or(config.server_port);

            let storage = FileStorage::new(config.storage_file.clone());
            let routes = storage.load_initial_state()?;

            server::run(routes, port, storage).await?;
        }

        cli::commands::Commands::Add {
            method,
            path,
            status,
            delay_ms,
            response,
            server,
        } => {
            // Validate that JSON is valid
            let body = serde_json::json!({
                "method": method,
                "path": path,
                "status": status,
                "delay_ms": delay_ms,
                "response": serde_json::from_str::<Value>(&response)?,
            });

            reqwest::Client::new()
                .post(format!("{server}/__admin/routes"))
                .json(&body)
                .send()
                .await?;

            println!("✅ Endpoint added");
        }

        cli::commands::Commands::List { server } => {
            let resp = reqwest::get(format!("{server}/__admin/routes"))
                .await?
                .json::<Value>()
                .await?;

            println!("{}", serde_json::to_string_pretty(&resp)?);
        }

        cli::commands::Commands::Shutdown { server } => {
            let resp = reqwest::Client::new()
                .post(format!("{server}/__admin/shutdown"))
                .send()
                .await?
                .json::<Value>()
                .await?;

            println!(
                "🛑 {}",
                resp.get("message")
                    .unwrap_or(&Value::String("Server shutdown".to_string()))
            );
        }
    }

    Ok(())
}
