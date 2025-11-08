# Pinyin Card Rust

A web API built with Rust, using Axum, Serde, Tokio, and MySQL.

## Features

- 🚀 Fast and async web server powered by Axum
- 📦 JSON serialization with Serde
- ⚡ Async runtime with Tokio
- 🔍 Multiple API endpoints
- 🗄️ MySQL database integration with SQLx
- 📑 Category management system

## Prerequisites

- Rust (1.70 or later)
- Cargo
- MySQL database

## Installation

1. Clone the repository and navigate to the project directory:

```bash
cd pinyin-card-rust
```

2. Create a `.env` file in the project root with your database configuration:

```bash
DATABASE_URL=mysql://username:password@localhost:3306/database_name
```

Replace `username`, `password`, `localhost:3306`, and `database_name` with your actual MySQL credentials.

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

## API Endpoints

### Health Check

- **GET** `/health` - Returns the health status of the server

### Categories

- **GET** `/categories` - Returns all categories from the database

Example response:

```json
[
  {
    "id": 1,
    "name_en": "Food",
    "name_pt": "Comida",
    "name_cht": "食物",
    "name_chs": "食物",
    "name_fr": "Nourriture",
    "name_it": "Cibo",
    "name_de": "Essen",
    "parent_category_id": null,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
]
```

## Dependencies

- **Axum**: Web framework for Rust
- **Tokio**: Async runtime
- **Serde**: Serialization/deserialization framework
- **Chrono**: Date and time library
- **SQLx**: Async SQL toolkit with MySQL support
- **dotenvy**: Environment variable loader

## License

MIT
