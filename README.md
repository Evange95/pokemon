# Pokemon

A Rust-based Pokemon API service that fetches Pokemon data from PokeAPI and provides fun translated descriptions using Shakespeare and Yoda translations.

## Quick Start

Get up and running in under 2 minutes:

```bash
# 1. Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Install cargo-make
cargo install --force cargo-make

# 3. Run the server (uses sensible defaults, no configuration needed)
cargo run
```

The server starts on `http://localhost:3000`. Test it:

```bash
curl http://localhost:3000/pokemon/pikachu
```

**That's it!** The app works out-of-the-box with default settings. Environment configuration is optional.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [API Documentation](#api-documentation)
- [Development](#development)
- [Docker](#docker)
- [Observability](#observability)
- [Production Considerations](#production-considerations)
- [Architecture](#architecture)
- [CI/CD](#cicd)

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- cargo-make for task automation (installed in setup steps)

## Installation

### 1. Install Rust

If you don't have Rust installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:

```bash
rustc --version
cargo --version
```

### 2. Install cargo-make

```bash
cargo install --force cargo-make
```

### 3. Clone and Build

```bash
# Clone the repository (if you haven't already)
git clone <repository-url>
cd pokemon

# Build the project
cargo make build

# Run the server
cargo run
```

The server will start on `http://0.0.0.0:3000`.

## Configuration

**Environment variables are optional** - the app uses sensible defaults. You only need to configure if you want to customize settings.

### Available Environment Variables

| Variable                    | Description                      | Default                           |
| --------------------------- | -------------------------------- | --------------------------------- |
| `HOST`                      | Server bind address              | `0.0.0.0`                         |
| `PORT`                      | Server port                      | `3000`                            |
| `POKE_API_BASE_URL`         | PokeAPI base URL                 | `https://pokeapi.co/api/v2`       |
| `FUN_TRANSLATIONS_BASE_URL` | Fun Translations API base URL    | `https://api.funtranslations.com` |
| `ENV`                       | Environment name (informational) | `development`                     |

### Setting Up Custom Configuration

**Option 1: Using .env file**

```bash
cp .env.example .env
# Edit .env with your preferred settings
```

**Option 2: Export environment variables**

```bash
export HOST=127.0.0.1
export PORT=8080
export ENV=production
cargo run
```

## API Documentation

### Endpoints

#### GET /pokemon/:name

Returns Pokemon information fetched from [PokeAPI](https://pokeapi.co).

**Request:**

```bash
curl http://localhost:3000/pokemon/pikachu
```

**Response (200 OK):**

```json
{
  "name": "pikachu",
  "description": "When several of these POKéMON gather, their electricity could build and cause lightning storms.",
  "habitat": "forest",
  "isLegendary": false
}
```

**Error Responses:**

- **404 Not Found** - Pokemon doesn't exist

  ```json
  {
    "error": "Pokemon 'unknown' not found"
  }
  ```

- **503 Service Unavailable** - PokeAPI is down

  ```json
  {
    "error": "Pokedex service is currently unavailable"
  }
  ```

- **500 Internal Server Error** - Invalid data from PokeAPI
  ```json
  {
    "error": "Invalid pokemon data: ..."
  }
  ```

#### GET /pokemon/translated/:name

Returns Pokemon information with a fun translated description.

**Translation Rules:**

- **Yoda** translation: Legendary Pokemon OR Pokemon with cave habitat
- **Shakespeare** translation: All other Pokemon
- **Graceful fallback**: Returns original description if translation fails (rate limits, service unavailable, etc.)

**Request (Legendary Pokemon):**

```bash
curl http://localhost:3000/pokemon/translated/mewtwo
```

**Response (200 OK - Yoda translation):**

```json
{
  "name": "mewtwo",
  "description": "Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was.",
  "habitat": "rare",
  "isLegendary": true
}
```

**Request (Normal Pokemon):**

```bash
curl http://localhost:3000/pokemon/translated/pikachu
```

**Response (200 OK - Shakespeare translation):**

```json
{
  "name": "pikachu",
  "description": "At which hour several of these pokémon gather,  their electricity couldst buildeth and cause lightning storms.",
  "habitat": "forest",
  "isLegendary": false
}
```

**Error Responses:**

Same as `/pokemon/:name` endpoint (404, 503, 500). Translation failures result in graceful fallback to original description, not errors.

**Note on Rate Limits:**

The Fun Translations API has rate limits (typically 5 requests/hour for free tier). When rate limited, the endpoint automatically falls back to the original untranslated description with a 200 OK response. Translation failures are logged but don't affect user experience.

## Development

### Available Tasks

Run tasks using `cargo make <task>`:

| Task                           | Description                                                            |
| ------------------------------ | ---------------------------------------------------------------------- |
| `cargo make build`             | Clean and build the project                                            |
| `cargo make test`              | Clean and run tests                                                    |
| `cargo make format`            | Format code with rustfmt                                               |
| `cargo make clean`             | Clean build artifacts                                                  |
| `cargo make clippy`            | Run clippy linter                                                      |
| `cargo make clippy-watch`      | Watch for changes and run clippy (requires cargo-watch)                |
| `cargo make test-watch`        | Watch for changes, run tests, build, and format (requires cargo-watch) |
| `cargo make docker-build`      | Build Docker image                                                     |
| `cargo make docker-run`        | Run Docker container (port 3000)                                       |
| `cargo make docker-run-custom` | Run Docker container (port 8080, production settings)                  |

### Development Workflow

**Basic workflow:**

```bash
# Build and run
cargo make build
cargo run

# Run tests
cargo make test

# Format code
cargo make format

# Run linter
cargo make clippy
```

**Active development with auto-reload:**

```bash
# Install cargo-watch (done automatically by cargo-make)
cargo make test-watch
```

This watches for file changes and automatically runs tests, rebuilds, and formats code.

**Watch mode for linting:**

```bash
cargo make clippy-watch
```

### Running Tests

```bash
cargo make test
```

Or directly with cargo:

```bash
cargo test
```

## Docker

### Build Docker Image

```bash
cargo make docker-build
```

Or directly with Docker:

```bash
docker build -t pokemon-api .
```

### Run Docker Container

**Default configuration (port 3000):**

```bash
cargo make docker-run
```

**Custom configuration (port 8080, production):**

```bash
cargo make docker-run-custom
```

**Using Docker directly:**

```bash
# Default settings
docker run -p 3000:3000 pokemon-api

# Custom settings
docker run -p 8080:8080 \
  -e HOST=0.0.0.0 \
  -e PORT=8080 \
  -e ENV=production \
  pokemon-api
```

### Docker Compose (Optional)

You can create a `docker-compose.yml` file for easier orchestration:

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
      - POKE_API_BASE_URL=https://pokeapi.co/api/v2
      - FUN_TRANSLATIONS_BASE_URL=https://api.funtranslations.com
```

Run with:

```bash
docker-compose up
```

## Observability

The application includes structured logging and tracing using the `tracing` crate.

### Features

- **HTTP Request/Response Tracing**: Automatic logging of all HTTP requests via `tower-http` TraceLayer
- **Domain Event Logging**: Business operations logged through the Logger port
- **Structured Context**: Logs include relevant context (pokemon name, habitat, error types, translation style, etc.)

Logs are written to stdout in a compact format.

### What Gets Logged

- Server startup and configuration
- Pokemon fetch requests and results
- Translation requests, successes, and failures
- Errors with relevant context
- HTTP request/response details

### Domain Events Examples

The application emits domain-oriented events via the `Logger` port, mapped to structured logs with business context.

**Success example (fetching `mewtwo`):**

```
INFO Pokemon fetch requested pokemon.fetch.requested=true pokemon.name=mewtwo
INFO Pokemon fetch succeeded pokemon.fetch.succeeded=true pokemon.name=mewtwo pokemon.habitat=rare pokemon.is_legendary=true
```

**Failure example (unknown pokemon):**

```
INFO Pokemon fetch requested pokemon.fetch.requested=true pokemon.name=nonexistent
ERROR Pokemon fetch failed pokemon.fetch.failed=true pokemon.name=nonexistent pokemon.failure.reason="not_found"
```

### Event Types

- `FetchRequested`: Emitted before calling the `Pokedex` port
- `FetchSucceeded`: Emitted with `name`, `habitat`, `is_legendary` on success
- `FetchFailed`: Emitted with `name` and `failure.reason` on domain error
- `TranslationRequested`: Emitted before calling the `Translator` port with translation style
- `TranslationSucceeded`: Emitted when translation succeeds with style used
- `TranslationFailed`: Emitted when translation fails with reason and fallback indicator

## Production Considerations

This section outlines the key improvements needed to make this API production-ready. The current implementation is designed for simplicity and demonstration, but a production deployment requires additional features.

### 1. Caching Layer (Redis)

**Why:** External API calls (PokeAPI, Fun Translations) are slow and have rate limits. Caching reduces latency and prevents hitting rate limits.

**Implementation Strategy:**

- **Redis for distributed caching**: Share cache across multiple API instances
- **Cache Pokemon data**: TTL of 24 hours (Pokemon data rarely changes)
- **Cache translations**: TTL of 7 days (translations are deterministic)
- **Cache invalidation**: Implement manual cache invalidation endpoints for emergencies

**Benefits:**

- Reduces PokeAPI calls (Pokemon data doesn't change frequently)
- Avoids Fun Translations rate limits for popular Pokemon
- Improves response times for cached data
- Reduces external dependency failures

### 2. Rate Limiting

**Why:** Protect the API from abuse, ensure fair usage, and prevent cascading failures.

**Response (429 Too Many Requests):**

```json
{
  "error": "Rate limit exceeded. Try again in 45 seconds.",
  "retryAfter": 45
}
```

### 3. Circuit Breaker Pattern

**Why:** Prevent cascading failures when external services (PokeAPI, Fun Translations) are down or slow.

**Implementation Strategy:**

- **Circuit states**:

  - **Closed**: Normal operation, requests pass through
  - **Open**: Service is failing, fail fast without calling external API
  - **Half-Open**: Test if service recovered with limited requests

- **Configuration**:

  - Open circuit after 5 consecutive failures or 50% error rate in 1 minute
  - Stay open for 30 seconds before attempting recovery
  - Use 3 test requests in half-open state

- **Fallback behavior**:
  - PokeAPI: Return cached data or 503 if no cache available
  - Fun Translations: Return untranslated description (already implemented)

**Benefits:**

- Protects against thundering herd when services recover
- Reduces wasted resources calling failing services
- Faster failures (fail fast instead of waiting for timeouts)

### 4. Status routes

**Implementation:**

- **Health checks**: `/health` endpoint for load balancer health checks
- **Readiness checks**: `/ready` endpoint checking external dependencies

### 5. Request Timeout & Retry Logic

**Why:** External APIs can be slow or hang, degrading user experience.

**Implementation:**

- **Request timeouts**:

  - PokeAPI: 5 second timeout
  - Fun Translations: 3 second timeout (fall back to original text)

- **Retry strategy**:
  - Retry 3 times with exponential backoff (100ms, 200ms, 400ms)
  - Only retry on network errors, not 4xx errors
  - Circuit breaker should trip before exhausting retries

### 6. Security Enhancements

**Why:** Production APIs need protection against attacks and data leaks.

**Implementation:**

- **HTTPS/TLS**: Terminate TLS at load balancer or reverse proxy (nginx)
- **CORS configuration**: Restrict allowed origins

## Architecture

The project follows a clean hexagonal (ports & adapters) architecture.

### Project Structure

```
src/
├── domain/          # Core business entities and domain events
│   ├── Pokemon model with type aliases
│   ├── Domain errors (PokemonErrors, TranslationErrors)
│   ├── Domain events (PokemonEvent with fetch/translation variants)
│   └── Translation types (TranslationStyle, TranslationFailureReason)
├── ports/           # Interfaces defining contracts
│   ├── Pokedex     # Fetch Pokemon data
│   ├── Translator  # Translate text to different styles
│   └── Logger      # Domain event observation
├── adapter/         # Implementations of ports
│   ├── PokeApiAdapter          # Fetches from PokeAPI (implements Pokedex)
│   ├── FunTranslationsAdapter  # Translates via Fun Translations API (implements Translator)
│   ├── TracingLogger           # Maps domain events to structured logs (implements Logger)
│   └── http                    # Axum HTTP adapter with routes
├── application/     # Use case orchestration
│   └── PokemonService
│       ├── get_pokemon()            # Fetch with event logging
│       └── get_translated_pokemon() # Fetch, translate, style selection, fallback
├── config.rs        # Environment configuration management
└── main.rs          # Dependency injection and server bootstrap
```

### Flow

```
HTTP Request → HTTP Adapter → Application Service → Port → Adapter Implementation
```

This layered separation keeps concerns isolated and makes it easy to swap implementations or add new adapters.

### Key Design Principles

- **Dependency Inversion**: Core domain doesn't depend on external services
- **Port & Adapter Pattern**: Clean separation between business logic and infrastructure
- **Domain Events**: Observable business operations for monitoring and debugging
- **Graceful Degradation**: Translation failures don't break the API
- **Configuration Flexibility**: Environment-based configuration with sensible defaults

## CI/CD

The project uses GitHub Actions for continuous integration:

- Automated testing on push and pull requests
- Code formatting checks
- Clippy linting
- Security audits
- Debug and release builds
