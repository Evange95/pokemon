# Copilot Instructions for Pokemon Project

## Project Overview

This is a Rust project using **cargo-make** for task automation. The codebase is in early stages with minimal implementation (single `main.rs` file currently).

## Build System & Workflows

### Task Automation with cargo-make

**Always use `cargo make <task>` instead of raw cargo commands.** The project enforces this pattern via `Makefile.toml`:

- `cargo make build` - Cleans first, then builds (dependencies enforce clean)
- `cargo make test` - Cleans first, then runs tests
- `cargo make format` - Auto-formats code with rustfmt
- `cargo make clippy-watch` - Watch mode with clippy linting
- `cargo make test-watch` - Watch mode that runs tests + build + format on file changes

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

- `src/main.rs` - Main application file containing Axum web server and API endpoints
- `Makefile.toml` - Task definitions (cargo-make configuration)
- `.github/workflows/ci.yml` - CI pipeline configuration
- `.github/copilot-instructions.md` - Instructions for AI agents (this file)
- `README.md` - User-facing documentation

## API Architecture

### Web Framework

The project uses **Axum 0.7** as the web framework, built on Tokio async runtime.

### Current Endpoints

- `GET /pokemon/:name` - Returns Pokemon data (currently static Mewtwo data)
  - Handler: `get_pokemon` function
  - Response: JSON with `PokemonResponse` struct
  - Fields are serialized to camelCase (e.g., `isLegendary`)

### Data Models

- `PokemonResponse` - Serializable struct with serde
  - Uses `#[serde(rename_all = "camelCase")]` for JSON naming
  - Fields: name, description, habitat, is_legendary

## Dependencies

Current dependencies in `Cargo.toml`:

- `axum = "0.7"` - Web framework
- `tokio = { version = "1", features = ["full"] }` - Async runtime
- `serde = { version = "1.0", features = ["derive"] }` - Serialization
- `serde_json = "1.0"` - JSON support
- `tower = { version = "0.4", features = ["util"] }` - Middleware (used in tests)

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
