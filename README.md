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
# Edit .env to customize HOST and PORT
```

## Configuration

The application can be configured using environment variables:

- `HOST` - Server bind address (default: `0.0.0.0`)
- `PORT` - Server port (default: `3000`)

### Environment Variables

Create a `.env` file in the project root (see `.env.example` for reference):

```bash
HOST=0.0.0.0
PORT=3000
```

Or set them directly:

```bash
export HOST=127.0.0.1
export PORT=8080
cargo run
```

## API

### Endpoints

#### GET /pokemon/:name

Returns Pokemon information. Currently returns static data for Mewtwo regardless of the requested name.

**Example Request:**

```bash
curl http://localhost:3000/pokemon/pikachu
```

**Response:**

```json
{
  "name": "mewtwo",
  "description": "It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.",
  "habitat": "rare",
  "isLegendary": true
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
docker run -p 8080:8080 -e HOST=0.0.0.0 -e PORT=8080 pokemon-api
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
