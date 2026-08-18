# Scafnix — Project Plan

> A Rust CLI for generating an opinionated TypeScript backend monorepo with Express.js, Prisma, Zod, shared configuration, ESLint, Prettier, and pnpm/Bun support.

---

## 1. Project Vision

Scafnix is a project scaffolding CLI written in Rust.

Its job is to generate a production-ready TypeScript backend monorepo from a predefined set of embedded templates.

The first version should intentionally be opinionated and small.

A generated project should contain:

- `apps/api` — Express.js REST API
- `packages/database` — Prisma database package
- `packages/schemas` — shared Zod schemas
- `packages/config` — shared configuration package
- ESLint
- Prettier
- TypeScript
- Pino logging
- `pino-http`
- `pino-pretty`
- `ts-add-js-extension`
- pnpm support
- Bun support

Future versions may add:

- Drizzle
- UI applications
- additional API frameworks
- additional database providers
- Docker
- authentication templates
- testing templates
- CI templates

The CLI should be designed so these can be added later without rewriting the core generator.

---

# 2. Primary Goal

The first usable command should eventually look like:

```bash
scafnix my-api
```

or:

```bash
scafnix create my-api
```

The CLI should ask the user for missing configuration:

```text
? Project name: my-api
? Package manager: pnpm
? Install dependencies? Yes
? Initialize Git repository? Yes
```

Then generate:

```text
my-api/
├── apps/
│   └── api/
├── packages/
│   ├── config/
│   ├── database/
│   └── schemas/
├── eslint.config.js
├── prettier.config.js
├── tsconfig.base.json
├── package.json
├── pnpm-workspace.yaml
└── README.md
```

The generated project should be runnable immediately.

Example:

```bash
cd my-api
pnpm dev
```

---

# 3. Non-Goals for Version 0.1

Do not attempt to support everything immediately.

Version `0.1` should NOT include:

- React
- Next.js
- Vue
- Drizzle
- Fastify
- Hono
- GraphQL
- authentication
- Docker
- Redis
- testing framework selection
- deployment configuration
- CI/CD generation
- multiple database engines
- plugin systems

The first version should prove that the scaffolding architecture works correctly.

---

# 4. Core Mental Model

Scafnix should be thought of as a project generator.

It performs this pipeline:

```text
CLI arguments
      │
      ▼
Interactive prompts
      │
      ▼
ProjectConfig
      │
      ▼
Validation
      │
      ▼
Template selection
      │
      ▼
Template rendering
      │
      ▼
Filesystem generation
      │
      ▼
Package manager commands
      │
      ▼
Post-generation tasks
      │
      ▼
Finished project
```

The important architectural rule is:

> User interaction and project generation must remain separate.

The generator should never depend directly on `inquire`.

This makes both interactive and non-interactive modes possible.

---

# 5. Technology Stack

## Rust CLI

Use:

```toml
inquire
indicatif
colored
```

Recommended additional dependencies:

```toml
clap
include_dir
thiserror
serde
serde_json
```

Possible later additions:

```toml
tempfile
assert_cmd
predicates
which
```

### Responsibilities

- `clap`
  - parse CLI commands and flags

- `inquire`
  - interactive prompts

- `indicatif`
  - spinners and progress indicators

- `colored`
  - terminal colors

- `include_dir`
  - embed template directories inside the executable

- `thiserror`
  - structured application errors

- `serde`
  - configuration/model serialization where useful

---

# 6. Generated Application Stack

## API

```text
Express.js
TypeScript
Pino
pino-http
pino-pretty
```

## Database

```text
Prisma
```

Drizzle should be represented architecturally but not implemented initially.

## Schemas

```text
Zod
```

## Shared Config

Initially:

```text
packages/config
```

The package can remain intentionally minimal.

Its purpose is to provide a place for shared configuration that applications and packages can import later.

## Tooling

```text
ESLint
Prettier
TypeScript
ts-add-js-extension
```

## Package Managers

```text
pnpm
bun
```

---

# 7. Proposed CLI Repository Structure

Start with:

```text
scafnix/
├── Cargo.toml
├── README.md
├── plan.md
│
├── src/
│   ├── main.rs
│   ├── cli/
│   │   ├── mod.rs
│   │   ├── args.rs
│   │   └── prompts.rs
│   │
│   ├── config/
│   │   ├── mod.rs
│   │   ├── project.rs
│   │   ├── package_manager.rs
│   │   └── database.rs
│   │
│   ├── generator/
│   │   ├── mod.rs
│   │   ├── project.rs
│   │   ├── base.rs
│   │   ├── api.rs
│   │   ├── database.rs
│   │   ├── schemas.rs
│   │   └── shared_config.rs
│   │
│   ├── template/
│   │   ├── mod.rs
│   │   ├── embedded.rs
│   │   └── renderer.rs
│   │
│   ├── package_manager/
│   │   ├── mod.rs
│   │   ├── pnpm.rs
│   │   └── bun.rs
│   │
│   ├── process/
│   │   └── mod.rs
│   │
│   ├── fs/
│   │   └── mod.rs
│   │
│   └── error.rs
│
├── templates/
│   ├── base/
│   ├── api/
│   │   └── express/
│   ├── database/
│   │   └── prisma/
│   ├── schemas/
│   │   └── zod/
│   └── config/
│
└── tests/
```

Do not create every module immediately.

Grow toward this structure incrementally.

---

# 8. Generated Monorepo Structure

The first generated project should target:

```text
my-project/
├── apps/
│   └── api/
│       ├── src/
│       │   ├── config/
│       │   ├── middleware/
│       │   ├── routes/
│       │   ├── utils/
│       │   ├── app.ts
│       │   └── server.ts
│       ├── package.json
│       └── tsconfig.json
│
├── packages/
│   ├── database/
│   │   ├── prisma/
│   │   │   └── schema.prisma
│   │   ├── src/
│   │   │   └── index.ts
│   │   ├── package.json
│   │   └── tsconfig.json
│   │
│   ├── schemas/
│   │   ├── src/
│   │   │   └── index.ts
│   │   ├── package.json
│   │   └── tsconfig.json
│   │
│   └── config/
│       ├── src/
│       │   └── index.ts
│       ├── package.json
│       └── tsconfig.json
│
├── .gitignore
├── eslint.config.js
├── prettier.config.js
├── tsconfig.base.json
├── package.json
├── pnpm-workspace.yaml
└── README.md
```

Bun-generated projects may differ where package-manager-specific files are required.

---

# 9. Core Domain Model

The CLI should convert every source of input into a single configuration object.

Example:

```rust
pub struct ProjectConfig {
    pub name: String,
    pub destination: PathBuf,
    pub package_manager: PackageManager,
    pub database: Database,
    pub install_dependencies: bool,
    pub initialize_git: bool,
}
```

Use enums instead of strings.

```rust
pub enum PackageManager {
    Pnpm,
    Bun,
}

pub enum Database {
    Prisma,
}
```

Future:

```rust
pub enum Database {
    Prisma,
    Drizzle,
}
```

Avoid configuration such as:

```rust
is_pnpm: bool,
is_bun: bool,
use_prisma: bool,
use_drizzle: bool,
```

Enums prevent invalid combinations.

---

# 10. CLI Architecture

The CLI layer should ONLY:

1. parse arguments
2. ask questions
3. build `ProjectConfig`
4. call the application/generator layer
5. display results or errors

It should not know how templates are copied.

Example flow:

```rust
fn main() -> Result<(), AppError> {
    let args = Cli::parse();

    let config = collect_project_config(args)?;

    generate_project(&config)?;

    Ok(())
}
```

Keep `main.rs` small.

---

# 11. Interactive and Non-Interactive Mode

Design both from the beginning.

Interactive:

```bash
scafnix
```

or:

```bash
scafnix my-api
```

Non-interactive:

```bash
scafnix my-api --package-manager pnpm --yes
```

Future example:

```bash
scafnix my-api \
  --package-manager bun \
  --database prisma \
  --no-git
```

Both paths MUST produce a `ProjectConfig`.

```text
CLI flags ──────────────┐
                       │
                       ▼
                  ProjectConfig
                       ▲
                       │
inquire prompts ───────┘
```

The generator only receives `ProjectConfig`.

---

# 12. Template Architecture

Do not generate every TypeScript file from large Rust strings.

Keep real files in:

```text
templates/
```

Example:

```text
templates/
├── base/
│   ├── package.json
│   ├── tsconfig.base.json
│   ├── eslint.config.js
│   ├── prettier.config.js
│   └── gitignore
│
├── api/
│   └── express/
│       ├── package.json
│       ├── tsconfig.json
│       └── src/
│           ├── app.ts
│           ├── server.ts
│           ├── middleware/
│           └── utils/
│
├── database/
│   └── prisma/
│       ├── package.json
│       ├── tsconfig.json
│       ├── prisma/
│       │   └── schema.prisma
│       └── src/
│           └── index.ts
│
├── schemas/
│   └── zod/
│       ├── package.json
│       ├── tsconfig.json
│       └── src/
│           └── index.ts
│
└── config/
    ├── package.json
    ├── tsconfig.json
    └── src/
        └── index.ts
```

---

# 13. Embedded Templates

Use `include_dir` to compile templates into the binary.

Example:

```rust
use include_dir::{include_dir, Dir};

static BASE_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/base");

static EXPRESS_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/api/express");

static PRISMA_TEMPLATE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/templates/database/prisma");
```

Conceptually:

```text
templates/*
     │
     │ cargo build
     ▼
┌───────────────────┐
│ scafnix executable│
│                   │
│ Rust code         │
│ +                 │
│ embedded templates│
└───────────────────┘
```

The installed executable should not require an external templates directory.

---

# 14. Template Extraction

Implement a reusable extraction function.

Conceptually:

```rust
fn extract_directory(
    template: &Dir,
    destination: &Path,
    context: &TemplateContext,
) -> Result<(), AppError>
```

For every directory:

1. create the corresponding destination directory
2. recurse into child directories

For every file:

1. determine destination path
2. create parent directory
3. read embedded bytes
4. determine whether file is text
5. render variables if text
6. write output

Binary assets should be copied unchanged.

---

# 15. Template Rendering

Start simple.

Supported variables could initially be:

```text
{{project_name}}
{{package_scope}}
{{package_manager}}
```

Example template:

```json
{
  "name": "{{project_name}}",
  "private": true
}
```

Renderer:

```rust
pub struct TemplateContext {
    pub project_name: String,
    pub package_scope: String,
    pub package_manager: String,
}
```

Initial rendering can use explicit replacements.

Do not implement a custom general-purpose template language.

If requirements become complicated later, evaluate a dedicated Rust template engine.

---

# 16. Package Naming Strategy

Decide this early.

Recommended generated package naming:

```text
@my-project/api
@my-project/database
@my-project/schemas
@my-project/config
```

For:

```text
my-project
```

generated package manifests could contain:

```json
{
  "name": "@my-project/database"
}
```

Normalize the project name before using it as a package scope.

Validation should reject or transform invalid npm package names.

---

# 17. Base Generator

The base generator is responsible for files common to every project.

It should generate:

```text
package.json
tsconfig.base.json
eslint.config.js
prettier.config.js
.gitignore
README.md
```

It should not generate Express-specific or Prisma-specific files.

Conceptual interface:

```rust
pub fn generate_base(
    config: &ProjectConfig,
    destination: &Path,
) -> Result<(), AppError>
```

---

# 18. Express Generator

The Express generator should create:

```text
apps/api
```

Initial API template should remain small.

Suggested files:

```text
apps/api/
├── src/
│   ├── config/
│   │   └── env.ts
│   ├── middleware/
│   │   ├── error-handler.ts
│   │   └── not-found.ts
│   ├── routes/
│   │   └── health.ts
│   ├── utils/
│   │   └── logger.ts
│   ├── app.ts
│   └── server.ts
├── package.json
└── tsconfig.json
```

Avoid generating a giant MVC architecture before the user needs it.

A scaffold should provide structure without producing unnecessary placeholder code.

---

# 19. Logging

Use:

```text
pino
pino-http
pino-pretty
```

Development logging should be human-readable.

Production logging should remain structured JSON.

Possible behavior:

```text
development
    ↓
pino-pretty

production
    ↓
JSON logs
```

The API should include:

- application logger
- HTTP request logging
- error logging

---

# 20. Prisma Database Package

Generate:

```text
packages/database/
├── prisma/
│   └── schema.prisma
├── src/
│   └── index.ts
├── package.json
└── tsconfig.json
```

The package should expose the Prisma client.

Keep the initial schema minimal.

Do not add application-specific models.

The user should receive a clean database foundation.

---

# 21. Schema Package

Generate:

```text
packages/schemas/
├── src/
│   └── index.ts
├── package.json
└── tsconfig.json
```

Use Zod.

Initially, the package may contain only exports and one small example if needed.

Avoid business-specific schemas.

---

# 22. Shared Config Package

Generate:

```text
packages/config/
├── src/
│   └── index.ts
├── package.json
└── tsconfig.json
```

It may intentionally be nearly empty.

Its purpose is to reserve a conventional location for values shared between applications/packages.

---

# 23. Package Manager Abstraction

Do not scatter package manager checks around the codebase.

Create:

```rust
pub enum PackageManager {
    Pnpm,
    Bun,
}
```

Then centralize behavior.

Possible methods:

```rust
impl PackageManager {
    pub fn executable(&self) -> &'static str;

    pub fn install_args(&self) -> &'static [&'static str];

    pub fn run_args(&self, script: &str) -> Vec<String>;

    pub fn display_name(&self) -> &'static str;
}
```

Package-manager-specific generation should also live here or in dedicated strategy modules.

---

# 24. pnpm Support

For pnpm, generate the required workspace configuration.

Example:

```yaml
packages:
  - "apps/*"
  - "packages/*"
```

Use pnpm for post-generation commands:

```bash
pnpm install
```

and where necessary:

```bash
pnpm db:generate
```

Do not hardcode `pnpm` into package scripts that should remain package-manager-neutral.

---

# 25. Bun Support

Bun support should have its own implementation.

Do not assume every pnpm behavior maps perfectly to Bun.

Verify:

- workspace handling
- lockfile behavior
- workspace dependency syntax
- script execution
- package installation
- lifecycle script behavior

The generated output should represent a genuinely supported Bun project rather than a pnpm project with commands renamed.

---

# 26. Process Execution

Create a central helper around:

```rust
std::process::Command
```

Example responsibility:

```rust
run_command(
    executable,
    args,
    working_directory,
)
```

It should:

1. execute command
2. capture or stream status
3. provide useful failure messages
4. preserve stderr when commands fail

Never hide useful package manager errors behind a spinner.

---

# 27. Progress UI

Use `indicatif`.

Possible generation output:

```text
✔ Validated project configuration
✔ Created project directory
✔ Generated base workspace
✔ Generated Express API
✔ Generated Prisma package
✔ Generated schema package
✔ Generated config package
✔ Configured pnpm
✔ Installed dependencies
✔ Generated Prisma client
✔ Initialized Git repository
```

Use spinners only for tasks that actually take noticeable time.

Examples:

```text
⠋ Installing dependencies...
```

Avoid excessive animation for instant filesystem operations.

---

# 28. Terminal Colors

Use `colored` sparingly.

Suggested semantic usage:

```text
green   = success
yellow  = warning
red     = failure
cyan    = commands/paths/highlights
```

The CLI should still be readable if colors are unavailable.

---

# 29. Validation

Perform validation before modifying the filesystem.

Validate:

- project name
- destination path
- whether destination already exists
- destination permissions
- package manager availability when installation is requested
- required runtime/tool availability if appropriate

Order:

```text
Collect configuration
        ↓
Validate everything possible
        ↓
Start modifying filesystem
```

Do not create half a project and then discover the project name was invalid.

---

# 30. Existing Directory Policy

For version `0.1`, keep behavior conservative.

If:

```text
./my-api
```

already exists and is non-empty, abort.

Do not overwrite files automatically.

Possible message:

```text
Error: destination "my-api" already exists and is not empty.
```

A future `--force` option can be considered later.

---

# 31. Error Architecture

Create one application-level error type.

Example:

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("invalid project name: {0}")]
    InvalidProjectName(String),

    #[error("destination already exists: {0}")]
    DestinationExists(PathBuf),

    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),

    #[error("command failed: {0}")]
    CommandFailed(String),

    #[error("template error: {0}")]
    Template(String),
}
```

Avoid large amounts of:

```rust
unwrap()
expect()
```

in normal runtime paths.

---

# 32. Failure Cleanup

Think carefully about partial generation.

Example:

```text
Create files       ✓
Install packages   ✗
```

Two reasonable policies exist.

## Policy A — Keep generated files

Useful because users can inspect/fix the dependency issue.

Display:

```text
Project files were generated successfully,
but dependency installation failed.
```

## Policy B — Roll back everything

Cleaner but potentially frustrating.

Recommended initial approach:

> Keep generated files if generation succeeded but a post-generation external command failed.

Clearly explain which step failed.

---

# 33. Git Initialization

Make Git initialization optional.

Interactive prompt:

```text
? Initialize a Git repository? Yes
```

Command:

```bash
git init
```

Optionally create initial commit in a later version.

Do not make a commit automatically in `0.1`.

---

# 34. Dependency Installation

Make installation optional.

Interactive:

```text
? Install dependencies? Yes
```

CLI:

```bash
scafnix my-api --no-install
```

If disabled, show the correct next command:

pnpm:

```text
cd my-api
pnpm install
pnpm dev
```

Bun:

```text
cd my-api
bun install
bun run dev
```

---

# 35. ts-add-js-extension

Packages compiled to JavaScript may require `.js` extensions in emitted ESM import paths.

The generated project should use `ts-add-js-extension` in the build pipeline where needed.

Keep this behavior centralized in package scripts or shared tooling rather than duplicating it manually in every source file.

The generator should not itself rewrite TypeScript import paths unless there is a strong reason to do so.

---

# 36. Development Milestones

Build Scafnix incrementally.

Each milestone should leave the project in a working state.

---

# Milestone 0 — Bootstrap Rust Project

Goal:

Create the CLI project.

Tasks:

1. Create repository.
2. Run:

```bash
cargo init
```

3. Add:

```toml
clap
thiserror
```

4. Create a minimal CLI.
5. Implement:

```bash
scafnix --help
scafnix --version
```

Success criteria:

```bash
cargo run -- --help
```

works correctly.

Do not add templates yet.

---

# Milestone 1 — Embed One TypeScript File

Goal:

Understand embedded resources completely.

Create:

```text
templates/
└── server.ts
```

Use:

```rust
include_str!()
```

Embed the file.

Command:

```bash
cargo run -- demo
```

should create:

```text
demo/
└── server.ts
```

Then:

1. run `cargo build`
2. temporarily move/delete the source template
3. run the already-built executable

The executable should still generate `server.ts`.

Success criteria:

You understand why the source template is no longer required at runtime.

---

# Milestone 2 — Embed a Directory

Goal:

Move from one file to a real template tree.

Add:

```toml
include_dir
```

Create:

```text
templates/base/
├── package.json
└── src/
    └── index.ts
```

Embed the directory.

Implement recursive extraction.

Generate:

```text
demo/
├── package.json
└── src/
    └── index.ts
```

Success criteria:

Any nested embedded directory can be extracted recursively.

---

# Milestone 3 — Template Renderer

Goal:

Support project-specific values.

Template:

```json
{
  "name": "{{project_name}}"
}
```

Create:

```rust
TemplateContext
```

Implement replacement.

Generate:

```bash
scafnix hello
```

Result:

```json
{
  "name": "hello"
}
```

Success criteria:

Text templates can use project metadata.

Binary files still copy unchanged.

---

# Milestone 4 — ProjectConfig

Goal:

Separate configuration from generation.

Create:

```rust
ProjectConfig
PackageManager
Database
```

Hardcode values temporarily if necessary.

Generator signature:

```rust
generate_project(&ProjectConfig)
```

Success criteria:

Generator contains no interactive prompt code.

---

# Milestone 5 — Generate Base Monorepo

Goal:

Generate only the root workspace.

Templates:

```text
package.json
tsconfig.base.json
eslint.config.js
prettier.config.js
.gitignore
README.md
```

Generate:

```text
demo/
├── apps/
├── packages/
├── package.json
├── tsconfig.base.json
├── eslint.config.js
├── prettier.config.js
└── README.md
```

Success criteria:

The root project metadata is valid.

---

# Milestone 6 — Express API Template

Goal:

Generate:

```text
apps/api
```

Implement:

- Express
- TypeScript
- health endpoint
- centralized error middleware
- 404 middleware
- Pino logger
- `pino-http`
- development `pino-pretty`

Example health route:

```text
GET /health
```

Success criteria:

After dependencies are installed:

```bash
pnpm dev
```

starts the API.

---

# Milestone 7 — Prisma Package

Goal:

Generate:

```text
packages/database
```

Include:

- Prisma schema
- exported client
- package manifest
- build config

Success criteria:

```bash
pnpm db:generate
```

works in a pnpm-generated project.

---

# Milestone 8 — Zod Schema Package

Goal:

Generate:

```text
packages/schemas
```

Success criteria:

The API can import from the package.

Example:

```ts
import { ... } from "@project/schemas";
```

---

# Milestone 9 — Shared Config Package

Goal:

Generate:

```text
packages/config
```

Keep implementation minimal.

Success criteria:

Other workspace packages can import it successfully.

---

# Milestone 10 — pnpm Integration

Goal:

Generate a fully functional pnpm project.

Implement:

- workspace file
- root scripts
- workspace dependencies
- install command
- development command
- build command
- database scripts

Success criteria:

For a generated project:

```bash
pnpm install
pnpm build
pnpm lint
pnpm format
```

all succeed.

---

# Milestone 11 — CLI Arguments with clap

Goal:

Support:

```bash
scafnix my-api
```

and:

```bash
scafnix create my-api
```

Choose one command style and keep it consistent.

Potential flags:

```text
--package-manager
--no-install
--no-git
--yes
```

Success criteria:

A project can be generated without `inquire` when sufficient flags are supplied.

---

# Milestone 12 — Interactive Prompts

Goal:

Add `inquire`.

Prompt only for information that is missing.

Example:

```text
? Project name:
? Package manager:
? Install dependencies?
? Initialize Git repository?
```

If project name was supplied:

```bash
scafnix my-api
```

do not ask for it again.

Success criteria:

Interactive and non-interactive modes produce equivalent `ProjectConfig` values.

---

# Milestone 13 — Progress UI

Goal:

Add `indicatif` and `colored`.

Keep generation readable and informative.

Success criteria:

Failures display their real cause and no child-process error output is lost.

---

# Milestone 14 — Bun Support

Goal:

Generate a real Bun-compatible workspace.

Implement package manager strategy separately from pnpm.

Test:

```bash
bun install
bun run build
bun run lint
bun run dev
```

Success criteria:

Bun output contains no accidental pnpm-only assumptions.

---

# Milestone 15 — Git Integration

Goal:

Optional:

```bash
git init
```

Check Git availability first.

Git failure should not destroy an otherwise valid generated project.

---

# Milestone 16 — End-to-End Tests

Goal:

Automatically verify generated projects.

Tests should generate projects into temporary directories.

Recommended tools:

```toml
tempfile
assert_cmd
predicates
```

Test cases:

1. generates expected directory tree
2. renders project name correctly
3. refuses non-empty destination
4. supports pnpm configuration
5. supports Bun configuration
6. `--no-install` does not invoke package manager
7. `--no-git` does not initialize Git
8. generated package manifests are valid JSON
9. expected workspace packages exist

Later CI integration tests can also run:

```bash
pnpm install
pnpm build
pnpm lint
```

against generated fixtures.

---

# 37. Testing Strategy

Use three testing levels.

## Unit Tests

Test pure logic:

- project name validation
- package scope normalization
- template replacement
- package manager command generation

## Generator Tests

Generate into temporary directories and inspect output.

Example:

```text
TempDir
   ↓
generate_project()
   ↓
assert files exist
```

## End-to-End Tests

Compile/run the actual CLI.

Example:

```bash
scafnix demo --package-manager pnpm --no-install --no-git
```

Assert the generated project tree.

---

# 38. Golden Fixture Testing

Consider maintaining known-good generated projects later.

Example:

```text
tests/
└── fixtures/
    └── expected-pnpm/
```

Generate a project and compare important files to the expected fixture.

Do not snapshot dependency lockfiles unless needed, because they change frequently.

---

# 39. Generated Project Validation

A release should not be considered successful merely because Scafnix compiles.

Its generated project must also work.

CI should eventually:

```text
Build Scafnix
      ↓
Generate pnpm project
      ↓
Install dependencies
      ↓
Lint
      ↓
Build
      ↓
Generate Bun project
      ↓
Install dependencies
      ↓
Lint
      ↓
Build
```

This is one of the most important tests in the entire project.

---

# 40. Template Development Workflow

Templates are application source code and should be treated like source code.

Do not blindly edit template files without validating them.

A useful development workflow:

```text
templates/
     ↓
Scafnix
     ↓
generated fixture
     ↓
package manager install
     ↓
lint/build/test
```

Consider adding a development command later that regenerates a local fixture.

---

# 41. Versioning

The CLI and embedded templates should ship together.

Example:

```text
Scafnix 0.3.0
      │
      └── embedded templates from 0.3.0
```

This gives deterministic generation.

Do not fetch the primary template from GitHub at runtime in early versions.

---

# 42. Offline Behavior

Because templates are embedded:

```text
scafnix my-api --no-install
```

should work without internet access.

Dependency installation naturally requires package registry access unless packages are already cached.

---

# 43. Release Builds

Use:

```bash
cargo build --release
```

Output:

```text
target/release/scafnix
```

Test the release binary separately from `cargo run`.

Ensure embedded resources are present in the final executable.

---

# 44. Cross-Platform Requirements

Scafnix should target:

- Linux
- macOS
- Windows

Avoid shell-specific assumptions.

Prefer:

```rust
std::process::Command
```

with executable and argument arrays instead of:

```text
sh -c "..."
```

Do not depend on Bash-specific syntax.

Use:

```rust
std::path::Path
PathBuf
```

instead of manually concatenating `/`.

---

# 45. Distribution

Potential future distribution methods:

## GitHub Releases

Provide compiled binaries:

```text
Linux x86_64
Linux ARM64
macOS ARM64
macOS x86_64
Windows x86_64
```

## Cargo

Potentially:

```bash
cargo install scafnix
```

subject to crate-name availability.

## Installer Script

Possible later convenience installer.

Do not make distribution work block early generator development.

---

# 46. README Goals

The Scafnix README should eventually explain:

1. what Scafnix is
2. what it generates
3. installation
4. quick start
5. CLI flags
6. generated project structure
7. supported package managers
8. supported database engines
9. roadmap
10. contributing

---

# 47. Agent Coding Rules

AI coding agents working on this project should follow these rules.

## Rule 1 — Follow the current milestone

Do not implement future roadmap functionality unless required by the current milestone.

## Rule 2 — Keep generation separate from prompts

No `inquire` calls inside generator modules.

## Rule 3 — Use typed configuration

Use enums and structs instead of unvalidated strings and boolean combinations.

## Rule 4 — Templates belong in `templates/`

Do not place large TypeScript source templates inside Rust strings.

Small test strings are acceptable.

## Rule 5 — Avoid premature abstractions

Do not implement plugin systems, dynamic template registries, dependency injection frameworks, or complex trait hierarchies before there is a concrete need.

## Rule 6 — Keep `main.rs` small

Application behavior belongs in modules.

## Rule 7 — Preserve command errors

Never swallow stdout/stderr needed to debug failed external commands.

## Rule 8 — Cross-platform first

Avoid shell-specific scripts inside Rust.

## Rule 9 — Generated code must compile

Any change to templates should be verified against a generated project.

## Rule 10 — No application-specific business logic

The template should remain generic.

Do not add users, products, authentication, billing, tasks, or similar domain models to the default scaffold.

---

# 48. Architecture Rules

The intended dependency direction is:

```text
CLI
 │
 ▼
ProjectConfig
 │
 ▼
Generator
 │
 ├── Template system
 ├── Filesystem
 └── PackageManager
        │
        ▼
     Process runner
```

Avoid reverse dependencies.

For example:

```text
generator → cli
```

should never be required.

---

# 49. Future Database Architecture

When Drizzle support is added, do not modify unrelated generator code everywhere.

Desired design:

```rust
match config.database {
    Database::Prisma => generate_prisma(...),
    Database::Drizzle => generate_drizzle(...),
}
```

Templates:

```text
templates/database/
├── prisma/
└── drizzle/
```

This should be a modular substitution.

---

# 50. Future UI Architecture

When UI support arrives:

```rust
pub enum Ui {
    None,
    React,
}
```

Templates:

```text
templates/ui/
└── react/
```

Generate to:

```text
apps/ui/
```

The backend generator should remain unchanged.

---

# 51. Future Feature Composition

Long-term, think of generation as:

```text
project =
    base
    + package manager
    + express
    + prisma
    + zod
    + shared config
```

Later:

```text
project =
    base
    + bun
    + express
    + drizzle
    + zod
    + shared config
    + react
```

Features should be composable rather than represented as complete duplicated starter projects.

---

# 52. Potential Future CLI UX

Example:

```text
$ scafnix my-api

  Scafnix

  Create a production-ready backend monorepo.

? Package manager
❯ pnpm
  bun

? Install dependencies? Yes
? Initialize Git? Yes

✔ Creating workspace
✔ Adding Express API
✔ Adding Prisma
✔ Adding shared Zod schemas
✔ Configuring ESLint and Prettier
✔ Installing dependencies
✔ Initializing Git

Created my-api

Next steps:

  cd my-api
  pnpm dev
```

Keep UX simple.

Do not prompt for decisions that Scafnix already makes opinionatedly.

---

# 53. Suggested Implementation Order

Use exactly this order unless a concrete issue requires changing it:

```text
1. Bootstrap Rust CLI
2. Embed one TypeScript file
3. Extract embedded directory
4. Render template variables
5. Create ProjectConfig
6. Generate base workspace
7. Generate Express API
8. Generate Prisma package
9. Generate Zod package
10. Generate config package
11. Make generated pnpm project compile
12. Add clap options
13. Add inquire prompts
14. Add indicatif progress UI
15. Add colored output
16. Add dependency installation
17. Add optional Git initialization
18. Add automated generator tests
19. Add generated-project CI validation
20. Add Bun support
21. Polish errors and UX
22. Prepare first release
```

Do not start Bun support before the pnpm-generated project is reliable.

---

# 54. First Development Session

The first coding session should have only one learning goal:

> Understand how a compiled Rust executable can contain and recreate a TypeScript file.

Create:

```text
templates/server.ts
```

Containing:

```ts
import express from "express";

const app = express();

app.get("/", (_req, res) => {
  res.json({ message: "Hello from Scafnix" });
});

app.listen(3000);
```

Embed it:

```rust
const SERVER_TEMPLATE: &str =
    include_str!("../templates/server.ts");
```

Write it:

```rust
std::fs::create_dir_all("demo")?;
std::fs::write("demo/server.ts", SERVER_TEMPLATE)?;
```

Build:

```bash
cargo build
```

Then execute the compiled binary directly.

After this works and the mechanism is understood, move to `include_dir`.

Do not skip this milestone.

---

# 55. Definition of Version 0.1 Complete

Scafnix `0.1` is complete when all of the following are true:

- [ ] CLI builds on Rust stable.
- [ ] `scafnix --help` works.
- [ ] Project name can be passed as an argument.
- [ ] Missing choices can be collected with `inquire`.
- [ ] Templates are embedded into the executable.
- [ ] No external template directory is required at runtime.
- [ ] Express API is generated.
- [ ] Prisma database package is generated.
- [ ] Zod schema package is generated.
- [ ] shared config package is generated.
- [ ] ESLint is configured.
- [ ] Prettier is configured.
- [ ] Pino logging is configured.
- [ ] `ts-add-js-extension` is integrated where needed.
- [ ] pnpm is fully supported.
- [ ] Bun is fully supported.
- [ ] dependency installation can be skipped.
- [ ] Git initialization can be skipped.
- [ ] existing non-empty directories are protected.
- [ ] meaningful failures are shown to the user.
- [ ] generated pnpm project builds successfully.
- [ ] generated Bun project builds successfully.
- [ ] generator tests run in CI.
- [ ] release binary works without source templates beside it.

---

# 56. Definition of Done for Every Feature

Before considering a feature complete:

- [ ] implementation is modular
- [ ] errors are handled without panic in normal usage
- [ ] tests exist where practical
- [ ] generated files are valid
- [ ] generated TypeScript builds
- [ ] linting passes
- [ ] formatting passes
- [ ] behavior works on filesystem paths safely
- [ ] no unnecessary future functionality was introduced
- [ ] `plan.md` is updated if architecture or project scope changes

---

# 57. Final Principle

Scafnix should remain opinionated.

The goal is not:

> Generate every possible Node.js backend.

The goal is:

> Generate one excellent backend monorepo quickly, predictably, and consistently.

Add configurability only when a real alternative becomes officially supported.

This keeps the codebase understandable, the generated projects reliable, and the CLI enjoyable to maintain.
