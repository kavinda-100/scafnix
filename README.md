# Scafnix

Scafnix is a Rust CLI for scaffolding opinionated TypeScript backend monorepos. It generates a ready-to-develop project with an API, shared packages, and the tooling needed to get started quickly.

Use Scafnix interactively when you want guidance, or pass every option inline when running in scripts, CI, or through an AI agent.

## Installation

> Coming soon. Scafnix has not been published yet.

## Usage

### Interactive mode

Run Scafnix without arguments and answer the prompts:

```bash
scafnix
```

You will be asked for the project name, package manager, API framework, database provider, whether to install dependencies, and whether to initialize Git.

### Inline arguments and AI agents

For automation and AI agents, provide configuration directly on the command line. This avoids interactive prompts and makes generation predictable:

```bash
scafnix my-api \
  --package-manager pnpm \
  --api express \
  --database prisma \
  --no-install \
  --no-git
```

Use `--yes` to accept defaults for any options you leave out:

```bash
scafnix my-api --yes
```

Available options:

```text
scafnix [OPTIONS] [PROJECT_NAME]

  -p, --package-manager <pnpm|bun>
  -a, --api <express>
  -d, --database <prisma|drizzle>
      --no-install
      --no-git
  -y, --yes
```

Run `scafnix --help` for the complete command reference.

## What Scafnix generates

Scafnix creates a TypeScript backend monorepo that includes:

- An Express API application in `apps/api`
- Shared configuration, schema, and database packages
- Prisma or Drizzle database support
- pnpm or Bun workspace support
- TypeScript, ESLint, and Prettier configuration
- Pino-based logging

## Technology

Scafnix is built with Rust and Cargo. Its CLI and generation workflow use:

- [Clap](https://crates.io/crates/clap) for command-line argument parsing
- [Inquire](https://crates.io/crates/inquire) for interactive prompts
- [Indicatif](https://crates.io/crates/indicatif) and [Colored](https://crates.io/crates/colored) for terminal feedback
- [IncludeDir](https://crates.io/crates/include_dir) to embed project templates in the binary
- [Serde](https://crates.io/crates/serde) and `serde_json` for data handling

## License

Scafnix is licensed under the [MIT License](LICENSE).

## Repository

Source code and issue tracking are available at [github.com/kavinda-100/scafnix](https://github.com/kavinda-100/scafnix).
