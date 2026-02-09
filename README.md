# 🎭 Mockie - Mock HTTP Server

> A configurable Mock HTTP server from CLI

## 🚀 Quick Start

```bash
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

## 📋 Features

- ✅ REST API to dynamically add/list endpoints
- ✅ Customizable JSON responses
- ✅ Custom HTTP status codes
- ✅ Configurable delays (simulate latency)
- ✅ JSON file persistence
- ✅ Hot reload of endpoints
- ✅ Intuitive CLI with `clap`

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

## 🤝 Contributing

This project is an educational example. Feel free to:
- Report bugs
- Propose improvements
- Create PRs with new features
- Create issues for discussions

## 📄 License

MIT
