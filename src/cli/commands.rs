use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// 🚀 Start the mock server
    Serve {
        #[arg(short, long)]
        port: Option<u16>,
    },

    /// ➕ Add a mock endpoint
    Add {
        #[arg(long)]
        method: String,

        #[arg(long)]
        path: String,

        #[arg(long, default_value_t = 200)]
        status: u16,

        #[arg(long, default_value_t = 0)]
        delay_ms: u64,

        /// JSON response (e.g., '{"ok":true}')
        #[arg(long)]
        response: String,

        /// URL of mock server
        #[arg(long, default_value = "http://localhost:3000")]
        server: String,
    },

    /// 📋 List registered endpoints
    List {
        #[arg(long, default_value = "http://localhost:3000")]
        server: String,
    },
}
