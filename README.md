# Pinyin Card Rust

A simple "Hello World" web API built with Rust, using Axum, Serde, and Tokio.

## Features

- 🚀 Fast and async web server powered by Axum
- 📦 JSON serialization with Serde
- ⚡ Async runtime with Tokio
- 🔍 Multiple API endpoints

## Prerequisites

- Rust (1.70 or later)
- Cargo

## Installation

Clone the repository and navigate to the project directory:

```bash
cd pinyin-card-rust
```

## Running the Server

To run the development server:

```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`

## API Endpoints

### GET /

Returns a simple "Hello, World!" message.

**Response:**

```json
{
  "message": "Hello, World!"
}
```

### GET /api/info

Returns information about the API.

**Response:**

```json
{
  "name": "Pinyin Card Rust API",
  "version": "0.1.0",
  "endpoints": ["/", "/api/info", "/api/health"]
}
```

### GET /api/health

Returns the health status of the API.

**Response:**

```json
{
  "status": "healthy",
  "timestamp": "2025-11-08T12:00:00Z"
}
```

## Testing the API

You can test the API using curl:

```bash
# Test the root endpoint
curl http://localhost:3000/

# Test the API info endpoint
curl http://localhost:3000/api/info

# Test the health check endpoint
curl http://localhost:3000/api/health
```

Or open your browser and navigate to:

- http://localhost:3000/
- http://localhost:3000/api/info
- http://localhost:3000/api/health

## Building for Production

To build an optimized release version:

```bash
cargo build --release
```

The binary will be available at `./target/release/pinyin-card-rust`

## Dependencies

- **Axum**: Web framework for Rust
- **Tokio**: Async runtime
- **Serde**: Serialization/deserialization framework
- **Chrono**: Date and time library

## License

MIT
