# Copilot Instructions for Pokemon Project

## Project Overview

This is a Rust project using **cargo-make** for task automation. The codebase follows a clean hexagonal architecture with separation of domain, ports, adapters, and application layers.

## Build System & Workflows

### Task Automation with cargo-make

**Always use `cargo make <task>` instead of raw cargo commands.** The project enforces this pattern via `Makefile.toml`:

- `cargo make build` - Cleans first, then builds (dependencies enforce clean)
- `cargo make test` - Cleans first, then runs tests
- `cargo make format` - Auto-formats code with rustfmt
- `cargo make clippy-watch` - Watch mode with clippy linting
- `cargo make test-watch` - Watch mode that runs tests + build + format on file changes
- `cargo make docker-build` - Build Docker image
- `cargo make docker-run` - Run Docker container with default configuration
- `cargo make docker-run-custom` - Run Docker container with custom port (8080)

### Development Workflow

For active development, use watch mode:

```bash
cargo make test-watch  # Recommended for continuous development
```

This automatically runs tests, builds, and formats on every save.

## Code Quality Standards

### Strict Compilation

CI enforces `RUSTFLAGS: -D warnings` - **all warnings are treated as errors**. Fix warnings immediately or they will fail CI.

### Required Checks

All PRs must pass:

1. **Tests** - `cargo test --verbose`
2. **Formatting** - `cargo fmt --all --check` (enforced, not just suggested)
3. **Clippy** - `cargo clippy --all-targets --all-features -- -D warnings` (zero warnings policy)
4. **Security Audit** - rustsec/audit-check runs on all changes
5. **Builds** - Both debug and release profiles must build successfully

### Pre-commit Checklist

Before committing, ensure:

```bash
cargo make format     # Auto-format code
cargo make clippy-watch  # Check for clippy issues
cargo make test       # Verify tests pass with clean build
```

## Project Structure

- `src/main.rs` - Application entry point, dependency injection, and server bootstrap
- `src/domain/` - Domain models and business entities (`Pokemon`, `PokemonErrors`)
- `src/ports/` - Port traits defining contracts (`Pokedex`, `Logger`)
- `src/adapter/` - Adapter implementations
  - `poke_api.rs` - `PokeApiAdapter` (real API integration)
  - `tracing_logger.rs` - `TracingLogger` (logging adapter using tracing crate)
  - `http/` - Axum HTTP adapter organized by route
    - `mod.rs` - Router assembly, exports `router()` function
    - `pokemon.rs` - Pokemon route handler, response DTO, and tests
- `src/application/` - Use case orchestration layer
  - `PokemonService` - Application service coordinating port calls with logging
- `src/config.rs` - Configuration module for environment variable management
- `Makefile.toml` - Task definitions (cargo-make configuration)
- `.github/workflows/ci.yml` - CI pipeline configuration
- `.github/copilot-instructions.md` - Instructions for AI agents (this file)
- `README.md` - User-facing documentation
- `.env.example` - Example environment variables file
- `Dockerfile` - Multi-stage Docker build configuration

## Configuration Management

### Environment Variables

The application uses a dedicated `config` module (`src/config.rs`) for configuration:

- **`Config` struct** - Holds server configuration (host, port)
- **`Config::from_env()`** - Loads config from environment variables with defaults
- **`Config::address()`** - Returns formatted bind address

#### Default Values

- `HOST` - Default: `0.0.0.0`
- `PORT` - Default: `3000`
- `POKE_API_BASE_URL` - Default: `https://pokeapi.co/api/v2`
- `ENV` - Default: `development` (informational; not currently used in app logic)

#### Pattern

Always use the `Config` module for configuration management. Keep main.rs lean by separating concerns into dedicated modules.

### Docker Configuration

The Dockerfile includes:

- Multi-stage build for optimized image size
- Environment variables with defaults (`ENV HOST=0.0.0.0`, `ENV PORT=3000`)
  - Optional: `ENV ENV=development`
- Non-root user for security
- Port exposure via `EXPOSE 3000`

## API Architecture

### Web Framework

The project uses **Axum 0.7** as the web framework, built on Tokio async runtime.

### Current Endpoints

- `GET /pokemon/:name` - Returns Pokemon data fetched from PokeAPI
  - Location: `src/adapter/http/pokemon.rs`
  - Handler: `get_pokemon_handler` function
  - Success Response (200): JSON with `PokemonResponse` struct
  - Error Responses:
    - 404 Not Found: When Pokemon doesn't exist
    - 503 Service Unavailable: When PokeAPI is down
    - 500 Internal Server Error: When data parsing fails
  - Fields are serialized to camelCase (e.g., `isLegendary`)

### Ports & Adapters Pattern

- **Domain Layer**: `Pokemon` struct and `PokemonErrors` enum in `src/domain/` represent core business entities and domain errors
- **Port (Driving)**: `Pokedex` trait in `src/ports/` defines the application interface (returns `Result<Pokemon, PokemonErrors>`)
- **Port (Observability)**: `Logger` trait in `src/ports/` defines domain logging interface for observability
  - Provides `info()`, `error()`, `warn()`, `debug()` methods with structured context
  - Infrastructure-agnostic - allows swapping logging implementations without changing domain code
- **Adapter (Driving)**: HTTP server in `src/adapter/http/` maps requests to application service
  - Organized by route: each route module exports `routes()` function
  - `http/mod.rs` merges all route modules and exports `router()` function
  - Maps `PokemonErrors` to HTTP status codes via `IntoResponse`
  - Integrated with tower-http TraceLayer for HTTP request/response logging
- **Application Layer**: `PokemonService` in `src/application/` coordinates use cases and port calls
  - Uses Logger port to emit domain events and trace execution flow
  - Logs successful operations and errors with structured context
- **Adapter (Driven - Data)**: `PokeApiAdapter` in `src/adapter/poke_api.rs` implements `Pokedex` by calling PokeAPI
  - Handles HTTP status codes: 200 (success), 404 (not found), 5xx (unavailable)
  - Maps infrastructure errors to domain errors
  - Uses fixtures from `src/adapter/fixtures/` for testing with wiremock
- **Adapter (Driven - Logging)**: `TracingLogger` in `src/adapter/tracing_logger.rs` implements `Logger` using tracing crate
  - Maps domain logging calls to structured tracing events
  - Formats context key-value pairs for tracing output
- **Composition**: `main.rs` constructs adapters, services, and injects dependencies
  - Initializes tracing subscriber for observability
  - Wires Logger and Pokedex adapters into PokemonService

Flow: HTTP Request → HTTP Adapter → Application Service → Port → Driven Adapter
Observability: Application Service → Logger Port → TracingLogger Adapter

Benefits:

- Clean separation between domain, ports, and adapters
- Application layer can add business logic, validation, transactions
- Easy to swap implementations (e.g., database, external API) without changing HTTP code
- Improves testability: each layer can be tested independently
- Follows dependency inversion principle
- Modular route organization: each endpoint lives in its own file for better maintainability
- Scalable structure: new routes can be added by creating new files and merging in `http/mod.rs`

### Data Models

- `Pokemon` (in `src/domain/`) - Core domain entity, serializable with serde
  - Uses `#[serde(rename_all = "camelCase")]` for JSON naming
  - Fields: name, description, habitat, is_legendary
  - Has `Debug` derive for error handling
- `PokemonErrors` (in `src/domain/`) - Domain error enum
  - `PokemonNotFound(String)` - Pokemon doesn't exist in the Pokedex
  - `PokedexUnavailable` - Pokedex service cannot be accessed (maps to infrastructure failures)
  - `InvalidPokemonData(String)` - Data from Pokedex is malformed or incomplete
- `PokemonResponse` (in `src/adapter/http/pokemon.rs`) - HTTP response DTO
  - Implements `From<Pokemon>` for domain → DTO conversion
  - Same fields as Pokemon, serialized to camelCase
- `ErrorResponse` (in `src/adapter/http/pokemon.rs`) - HTTP error response DTO
  - Contains `error` field with human-readable error message
  - Implements `IntoResponse` via `PokemonErrors`

### HTTP Route Organization Pattern

Each route module in `src/adapter/http/` follows this pattern:

1. **Response DTOs** - Define at top of file with serde serialization
2. **Handler Functions** - Private async functions using Axum extractors
3. **Route Factory** - Public `routes(service)` function returning `Router<Arc<PokemonService>>`
4. **Tests** - Integration tests in same file for locality

Example: `pokemon.rs` contains `PokemonResponse`, `get_pokemon_handler`, `routes()`, and tests.

To add a new route:

1. Create `src/adapter/http/new_route.rs` following the pattern above
2. Add `mod new_route;` to `src/adapter/http/mod.rs`
3. Merge routes in `router()` function: `.merge(new_route::routes(service))`

## Dependencies

Current dependencies in `Cargo.toml`:

- `axum = "0.7"` - Web framework
- `tokio = { version = "1", features = ["full"] }` - Async runtime
- `serde = { version = "1.0", features = ["derive"] }` - Serialization
- `serde_json = "1.0"` - JSON support
- `tower = { version = "0.4", features = ["util"] }` - Middleware (used in tests)
- `tower-http = { version = "0.5", features = ["trace"] }` - HTTP middleware with tracing support
- `dotenvy = "0.15"` - .env file support for local development
- `reqwest = { version = "0.11", features = ["json"] }` - HTTP client for API calls
- `async-trait = "0.1"` - Async trait support
- `tracing = "0.1"` - Structured logging and diagnostics
- `tracing-subscriber = { version = "0.3", features = ["env-filter"] }` - Tracing subscriber for collecting events

Dev dependencies:

- `wiremock = "0.6"` - HTTP mocking for adapter tests

When adding dependencies:

- Re-run security audit: `cargo audit` (requires `cargo install cargo-audit`)
- CI will automatically check for vulnerabilities via rustsec
- Update this documentation section

## Development Methodology

### Test-Driven Development (TDD)

**Always follow the Red-Green-Refactor cycle:**

1. **Red** - Write failing tests first (`cargo make test`)
2. **Green** - Implement the simplest solution to make tests pass
3. **Refactor** - Improve code quality while keeping tests green

Never implement features without tests first. The simplest implementation that passes tests is preferred over premature optimization.

### Commit Workflow

After completing the refactor phase, **ALWAYS commit using Conventional Commits**:

```bash
# Format: <type>(<scope>): <subject>
git commit -m "feat: add pokemon data parser"
git commit -m "fix(api): handle empty response gracefully"
git commit -m "refactor: simplify error handling logic"
git commit -m "test: add edge cases for evolution chains"
```

Types: `feat`, `fix`, `refactor`, `test`, `docs`, `chore`, `perf`, `style`

### Testing Patterns

**Adapter Tests with Fixtures:**

The PokeAPI adapter tests use wiremock for mocking HTTP responses and load fixture data from JSON files:

- **Fixture Location**: `src/adapter/fixtures/`
  - `mewtwo.json` - Full PokeAPI response for Mewtwo
  - `pikachu.json` - Full PokeAPI response for Pikachu
- **Loading Fixtures**: Use `include_str!("fixtures/pokemon.json")` to embed at compile time
- **Mock Server Pattern**: Create wiremock `MockServer` with appropriate responses for each test scenario
- **Error Scenarios**: Test 404 Not Found, 503 Service Unavailable, and invalid JSON parsing

Example test structure:

```rust
#[tokio::test]
async fn test_adapter_with_fixture() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/pokemon-species/mewtwo"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_string(include_str!("fixtures/mewtwo.json")))
        .mount(&mock_server).await;

    let adapter = PokeApiAdapter::new(mock_server.uri());
    let result = adapter.get_pokemon("mewtwo".to_string()).await;
    assert!(result.is_ok());
}
```

### Maintaining Documentation

**CRITICAL: Always update documentation after making changes. This is not optional.**

#### Files to Update

1. **`.github/copilot-instructions.md` (this file)** - Update whenever:

   - Adding new architectural patterns or conventions
   - Introducing new tools, dependencies, or workflows
   - Creating new API endpoints or data models
   - Establishing project-specific patterns
   - Making significant structural changes
   - Adding new development practices or standards

2. **`README.md`** - Update whenever:
   - Adding new API endpoints
   - Changing public-facing functionality
   - Adding new user-facing features
   - Modifying setup or installation steps
   - Adding new environment variables or configuration

#### Documentation Standards

- Document **what** was added and **why** it matters
- Include code examples for new patterns or APIs
- Update both user documentation (README) and developer documentation (copilot-instructions)
- Keep documentation in sync with actual implementation
- Documentation updates should be part of the same commit as the code changes

**Remember: Outdated documentation is worse than no documentation. Future AI agents and developers rely on these files to understand the project quickly and accurately.**

## Common Pitfalls

1. **Don't use raw cargo commands** - Use `cargo make` tasks to ensure consistency with CI
2. **Format before committing** - CI will fail on unformatted code
3. **Watch for warnings** - They're treated as errors in CI with `-D warnings`
4. **Clean builds matter** - Tasks like `build` and `test` enforce clean state via dependencies
5. **Don't skip tests** - Write tests first, implement second, refactor third
