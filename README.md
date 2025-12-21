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

## CI/CD

The project uses GitHub Actions for continuous integration:

- Automated testing on push and pull requests
- Code formatting checks
- Clippy linting
- Security audits
- Debug and release builds
