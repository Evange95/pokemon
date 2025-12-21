# Pokemon

A Rust project built with cargo-make for task automation.

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- cargo-make for task automation

## Setup

1. **Install Rust** (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Install cargo-make**:

```bash
cargo install --force cargo-make
```

3. **Configure environment variables** (optional):

```bash
cp .env.example .env
# Edit .env to customize HOST, PORT, and ENV
```

## Configuration

The application can be configured using environment variables:

- `HOST` - Server bind address (default: `0.0.0.0`)
- `PORT` - Server port (default: `3000`)
- `POKE_API_BASE_URL` - PokeAPI base URL (default: `https://pokeapi.co/api/v2`)
- `ENV` - Environment name (default: `development`). Currently informational for deployment; the server behavior does not change based on this value yet.

### Environment Variables

Create a `.env` file in the project root (see `.env.example` for reference):

```bash
HOST=0.0.0.0
PORT=3000
POKE_API_BASE_URL=https://pokeapi.co/api/v2
ENV=development
```

Or set them directly:

```bash
export HOST=127.0.0.1
export PORT=8080
export ENV=production
cargo run
```

## API

### Endpoints

#### GET /pokemon/:name

Returns Pokemon information fetched from the [PokeAPI](https://pokeapi.co).

**Example Request:**

```bash
curl http://localhost:3000/pokemon/pikachu
```

**Response:**

```json
{
  "name": "pikachu",
  "description": "When several of these POKéMON gather, their electricity could build and cause lightning storms.",
  "habitat": "forest",
  "isLegendary": false
}
```

**Error Responses:**

- **404 Not Found** - Pokemon doesn't exist:

  ```json
  {
    "error": "Pokemon 'unknown' not found"
  }
  ```

- **503 Service Unavailable** - PokeAPI is down:

  ```json
  {
    "error": "Pokedex service is currently unavailable"
  }
  ```

- **500 Internal Server Error** - Invalid data:
  ```json
  {
    "error": "Invalid pokemon data: ..."
  }
  ```

## Development

### Available Tasks

Run tasks using `cargo make <task>`:

- `cargo make build` - Clean and build the project
- `cargo make test` - Clean and run tests
- `cargo make format` - Format code with rustfmt
- `cargo make clean` - Clean build artifacts
- `cargo make clippy-watch` - Watch for changes and run clippy
- `cargo make test-watch` - Watch for changes and run tests, build, and format
- `cargo make docker-build` - Build Docker image
- `cargo make docker-run` - Run Docker container (default port 3000)
- `cargo make docker-run-custom` - Run Docker container (custom port 8080)

### Quick Start

Build and run the server:

```bash
cargo make build
cargo run
```

The server will start on `http://0.0.0.0:3000`.

Run tests:

```bash
cargo make test
```

Format code:

```bash
cargo make format
```

### Development Workflow

For active development with auto-reload:

```bash
cargo make test-watch
```

This will automatically run tests, rebuild, and format your code on every file change.

## Architecture

The project follows a clean hexagonal (ports & adapters) architecture:

- **Domain (`src/domain/`)**: Core business entities (`Pokemon` model)
- **Ports (`src/ports/`)**: Interfaces defining contracts (`Pokedex` trait)
- **Adapters (`src/adapter/`)**: Implementations of ports
  - `StaticPokemonApi`: Returns static Mewtwo data (implements `Pokedex`)
  - `http`: Axum HTTP adapter
- **Application (`src/application/`)**: Use case orchestration (`PokemonService`)
- **Config (`src/config.rs`)**: Environment configuration management
- **Main (`src/main.rs`)**: Dependency injection and server bootstrap

Flow: HTTP adapter → Application service → Port → Adapter implementation

This layered separation keeps concerns isolated and makes it easy to swap implementations or add new adapters.

## Docker

### Build Docker Image

Using cargo-make:

```bash
cargo make docker-build
```

Or directly with Docker:

```bash
docker build -t pokemon-api .
```

### Run Docker Container

With default configuration (port 3000):

```bash
cargo make docker-run
```

Or with custom port:

```bash
cargo make docker-run-custom  # Runs on port 8080
```

Using Docker directly:

```bash
# Default
docker run -p 3000:3000 pokemon-api

# Custom environment variables
docker run -p 8080:8080 -e HOST=0.0.0.0 -e PORT=8080 -e ENV=production pokemon-api
```

### Docker Compose (optional)

Create a `docker-compose.yml`:

```yaml
services:
  pokemon-api:
    build: .
    ports:
      - "3000:3000"
    environment:
      - HOST=0.0.0.0
      - PORT=3000
      - ENV=development
```

Run with:

```bash
docker-compose up
```

## CI/CD

The project uses GitHub Actions for continuous integration:

- Automated testing on push and pull requests
- Code formatting checks
- Clippy linting
- Security audits
- Debug and release builds
