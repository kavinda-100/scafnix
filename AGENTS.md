# Repository Guidelines

## Project Structure & Module Organization

Scafnix is a Rust command-line project managed by Cargo. Application code is in `src/`; `src/main.rs` is the current executable entry point. Add focused modules under `src/` as the CLI grows (for example, `src/commands.rs` or `src/templates.rs`) and declare them from the appropriate parent module. `Cargo.toml` defines package metadata and dependencies, while `Cargo.lock` pins resolved versions and should be committed. The project overview is currently in `READEME.md` (note the spelling).

## Build, Test, and Development Commands

- `cargo run` — compile and run the CLI locally.
- `cargo build` — build a debug executable in `target/debug/`.
- `cargo build --release` — produce an optimized binary in `target/release/`.
- `cargo test` — run the test suite; it succeeds even when no tests are defined.
- `cargo fmt --check` — verify Rust formatting; use `cargo fmt` to apply it.
- `cargo clippy -- -D warnings` — surface lint issues and treat warnings as failures.

## Coding Style & Naming Conventions

Follow standard Rust formatting through `rustfmt`; use four-space indentation and run `cargo fmt` before committing. Use `snake_case` for functions, modules, variables, and file names; `PascalCase` for structs, enums, and traits; and `SCREAMING_SNAKE_CASE` for constants. Prefer small, single-purpose functions and return contextual `anyhow` errors from command boundaries. Keep user-facing CLI text concise and consistent.

## Testing Guidelines

Place unit tests in a `#[cfg(test)] mod tests` block alongside the code being tested. Add end-to-end CLI behavior tests in `tests/` when commands, prompts, or filesystem output need validation. Name tests after observable behavior, such as `creates_project_directory`. Run `cargo test` before opening a pull request. There is no coverage threshold configured; add tests for new behavior and regressions.

## Commit & Pull Request Guidelines

Existing history uses short, lowercase, imperative summaries (for example, `init the project and installed dependencies.`). Continue that style: `add project template command` or `fix invalid config error`. Keep commits focused. Pull requests should explain the change and testing performed, link any relevant issue, and include sample terminal output or screenshots when user-visible CLI behavior changes.
