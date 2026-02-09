# 🎭 Mockie - Mock HTTP Server

> A configurable Mock HTTP server from CLI

## 🚀 Quick Start

```bash
# Show help
cargo run -- help

# Start the server
cargo run -- serve --port 3000

# In another terminal: add an endpoint
cargo run -- add \
  --method GET \
  --path /users \
  --response '{"users":[{"id":1,"name":"John"}]}'

# List endpoints
cargo run -- list

# Test it
curl http://localhost:3000/users
```

## 📦 Installation

### Option 1: Automatic Installation (Recommended)

#### Windows
```bash
# Download and run the installer in cmd
install.bat

# Or use Python installer
python install.py
```

#### macOS/Linux
```bash
# Make script executable
chmod +x install.sh

# Run installer
./install.sh

# Or use Python installer
python3 install.py
```

### Option 2: Manual Installation

1. Build the release binary
```bash
cargo build --release
```

2. Copy the executable to your system PATH:
   - **Windows**: `target\release\mockie.exe` → Add to PATH
   - **macOS/Linux**: `target/release/mockie` → `/usr/local/bin/`

3. Verify installation
```bash
mockie --version
```

### Option 3: From Source
```bash
# Clone and build
git clone <repo>
cd mockie
cargo build --release
./target/release/mockie serve
```

> 📚 See [INSTALL.md](INSTALL.md) for detailed installation instructions

## 📋 Features

- ✅ REST API to dynamically add/list endpoints
- ✅ Customizable JSON responses
- ✅ Custom HTTP status codes
- ✅ Configurable delays (simulate latency)
- ✅ JSON file persistence
- ✅ Hot reload of endpoints
- ✅ Intuitive CLI with `clap`
- ✅ Built-in help command

## 🎯 Available Commands

```bash
# Show help
cargo run -- help
# or
cargo run -- serve --help
cargo run -- add --help
cargo run -- list --help
cargo run -- shutdown --help

# Start mock server
cargo run -- serve [--port PORT]

# Add endpoint
cargo run -- add \
  --method METHOD \
  --path PATH \
  [--status STATUS] \
  [--delay_ms DELAY] \
  --response RESPONSE \
  [--server URL]

# List endpoints
cargo run -- list [--server URL]

# Shutdown server
cargo run -- shutdown [--server URL]
```

## 🛠️ Development

```bash
# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Tests
cargo test

# Optimized build
cargo build --release

# Documentation
cargo doc --open
```

## 📦 REST API

### Admin API

#### Add endpoint
```bash
POST /__admin/routes
Content-Type: application/json

{
  "method": "GET",
  "path": "/api/users",
  "status": 200,
  "delay_ms": 100,
  "response": {"users": []}
}
```

#### List endpoints
```bash
GET /__admin/routes

Response:
[
  {
    "method": "GET",
    "path": "/api/users",
    "status": 200,
    "delay_ms": 100
  }
]
```

#### Shutdown server
```bash
POST /__admin/shutdown

Response:
{
  "message": "Server shutting down..."
}
```

### Mock Endpoints

Any request that is not to `/__admin/*` is treated as mock:

```bash
GET /api/users
# Returns the configured response

DELETE /custom
# 404 if not configured
```

## 🔧 Environment Variables

```bash
MOCKIE_PORT=8080           # Port (default: 3000)
MOCKIE_STORAGE=./data.json # Storage file (default: routes.json)
MOCKIE_LOG_LEVEL=debug     # Log level (default: info)
```

## 📝 Usage Examples

```bash
# Start mock REST server
cargo run -- serve

# Add endpoint with delay
cargo run -- add \
  --method GET \
  --path /slow \
  --response '{"status":"processing"}' \
  --delay_ms 2000

# Endpoint that returns error
cargo run -- add \
  --method POST \
  --path /users \
  --status 500 \
  --response '{"error":"Internal Server Error"}'

# POST request
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json"

# GET request
curl http://localhost:3000/slow
```

## 🛑 Shutdown

```bash
# Shutdown using CLI
cargo run -- shutdown

# Shutdown using REST API
curl -X POST http://localhost:3000/__admin/shutdown
```

## 🤝 Contributing

This project is an educational example. Feel free to:
- Report bugs
- Propose improvements
- Create PRs with new features
- Create issues for discussions

## 📄 License

MIT
