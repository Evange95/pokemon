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
- `FUN_TRANSLATIONS_BASE_URL` - Fun Translations API base URL (default: `https://api.funtranslations.com`)
- `ENV` - Environment name (default: `development`). Currently informational for deployment; the server behavior does not change based on this value yet.

## Observability

The application includes structured logging and tracing using the `tracing` crate:

- **HTTP Request/Response Tracing**: Automatic logging of all HTTP requests via `tower-http` TraceLayer
- **Domain Event Logging**: Business operations are logged through the Logger port
- **Structured Context**: Logs include relevant context (pokemon name, habitat, error types, translation style, etc.)

Logs are written to stdout in a compact format. The application logs:

- Server startup and configuration
- Pokemon fetch requests and results
- Translation requests, successes, and failures
- Errors with relevant context
- HTTP request/response details

### Domain Events (Examples)

The application emits domain-oriented events via the `Logger` port. These are mapped to structured logs with clear business context.

Success example (fetching `mewtwo`):

```
INFO Pokemon fetch requested pokemon.fetch.requested=true pokemon.name=mewtwo
INFO Pokemon fetch succeeded pokemon.fetch.succeeded=true pokemon.name=mewtwo pokemon.habitat=rare pokemon.is_legendary=true
```

Failure example (unknown pokemon):

```
INFO Pokemon fetch requested pokemon.fetch.requested=true pokemon.name=nonexistent
ERROR Pokemon fetch failed pokemon.fetch.failed=true pokemon.name=nonexistent pokemon.failure.reason="not_found"
```

Event mapping:

- `FetchRequested`: emitted before calling the `Pokedex` port
- `FetchSucceeded`: emitted with `name`, `habitat`, `is_legendary` on success
- `FetchFailed`: emitted with `name` and `failure.reason` on domain error
- `TranslationRequested`: emitted before calling the `Translator` port with translation style
- `TranslationSucceeded`: emitted when translation succeeds with style used
- `TranslationFailed`: emitted when translation fails with reason and fallback indicator

### Environment Variables

Create a `.env` file in the project root (see `.env.example` for reference):

```bash
HOST=0.0.0.0
PORT=3000
POKE_API_BASE_URL=https://pokeapi.co/api/v2
FUN_TRANSLATIONS_BASE_URL=https://api.funtranslations.com
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

#### GET /pokemon/translated/:name

Returns Pokemon information with a translated description. Translation style is automatically selected:

- **Yoda** translation for legendary Pokemon or Pokemon with cave habitat
- **Shakespeare** translation for all other Pokemon
- Falls back to original description if translation fails (rate limit, service unavailable, etc.)

**Example Request:**

```bash
curl http://localhost:3000/pokemon/translated/mewtwo
```

**Response (Legendary Pokemon - Yoda translation):**

```json
{
  "name": "mewtwo",
  "description": "Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was.",
  "habitat": "rare",
  "isLegendary": true
}
```

**Response (Normal Pokemon - Shakespeare translation):**

```bash
curl http://localhost:3000/pokemon/translated/pikachu
```

```json
{
  "name": "pikachu",
  "description": "At which hour several of these pokémon gather,  their electricity couldst buildeth and cause lightning storms.",
  "habitat": "forest",
  "isLegendary": false
}
```

**Graceful Fallback:**

If the Fun Translations API is unavailable or rate-limited, the endpoint returns the original untranslated description with a 200 OK response. Translation failures are logged but do not affect the user experience.

**Error Responses:**

Same as the `/pokemon/:name` endpoint (404, 503, 500).

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

- **Domain (`src/domain/`)**: Core business entities and domain events
  - `Pokemon` model with type aliases
  - Domain errors: `PokemonErrors`, `TranslationErrors`
  - Domain events: `PokemonEvent` with fetch and translation variants
  - Translation types: `TranslationStyle`, `TranslationFailureReason`
- **Ports (`src/ports/`)**: Interfaces defining contracts
  - `Pokedex`: Fetch Pokemon data
  - `Translator`: Translate text to different styles
  - `Logger`: Domain event observation
- **Adapters (`src/adapter/`)**: Implementations of ports
  - `PokeApiAdapter`: Fetches Pokemon from PokeAPI (implements `Pokedex`)
  - `FunTranslationsAdapter`: Translates descriptions via Fun Translations API (implements `Translator`)
  - `TracingLogger`: Maps domain events to structured logs (implements `Logger`)
  - `http`: Axum HTTP adapter with routes
- **Application (`src/application/`)**: Use case orchestration (`PokemonService`)
  - `get_pokemon()`: Fetch Pokemon with event logging
  - `get_translated_pokemon()`: Fetch and translate with style selection and fallback
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
