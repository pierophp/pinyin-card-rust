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
