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

- `src/main.rs` - Single entry point (project is in early stages)
- `Makefile.toml` - Task definitions (cargo-make configuration)
- `.github/workflows/ci.yml` - CI pipeline configuration

## Dependencies

Currently **zero external dependencies** in `Cargo.toml`. When adding dependencies:

- Re-run security audit: `cargo audit` (requires `cargo install cargo-audit`)
- CI will automatically check for vulnerabilities via rustsec

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

**Update `.github/copilot-instructions.md` whenever:**

- Adding new architectural patterns or conventions
- Introducing new tools or workflows
- Establishing project-specific patterns
- Making significant structural changes

Keep instructions current so future AI agents can be immediately productive.

## Common Pitfalls

1. **Don't use raw cargo commands** - Use `cargo make` tasks to ensure consistency with CI
2. **Format before committing** - CI will fail on unformatted code
3. **Watch for warnings** - They're treated as errors in CI with `-D warnings`
4. **Clean builds matter** - Tasks like `build` and `test` enforce clean state via dependencies
5. **Don't skip tests** - Write tests first, implement second, refactor third
