# xg — Safe Git with Supercharged Project Starters

xg is a developer tool that provides Git safety features and project templating capabilities. It acts as a wrapper around Git commands with built-in safeguards, and includes generators for quickly scaffolding new projects.

Current focus is on Go project generation with framework-specific templates.

## Features

- **Git Safety Layer** – Prevent accidental commits of sensitive files, build artifacts, or IDE configuration
- **Project Generators** – Bootstrap new projects with production-ready structure and configurations
- **Go Support** – Generate REST API projects using Gin or Echo frameworks

## Status

| Component | Status | Notes |
| --- | --- | --- |
| Go Project Generator | Working | Generates complete projects with configs, dependencies, and build setup |
| Git Wrapper | Planned | Pass-through shim with pre-commit safety checks |
| Releases | Planned | Binary packaging and installation scripts |

## Usage

Generate a new Go project:

```bash
cargo run -- create-go-app \
  --project my-api \
  --rest-framework gin
```

This creates a complete Go project structure with:
- Idiomatic folder layout (`cmd/`, `config/`, `internal/`)
- Environment-specific configuration files
- Framework bootstrap code
- Go module initialization and dependency management

## Project Structure

Generated Go projects include:

```
my-api/
├── cmd/
│   └── main.go                 # Application entry point
├── config/
│   ├── config.development.json # Development configuration
│   └── config.production.json  # Production configuration
├── internal/
│   └── config/
│       └── config.go           # Configuration loading and validation
└── go.mod                      # Go module file
```

## Development

```bash
git clone https://github.com/nishujangra/xg
cd xg
cargo build
```

Key directories:
- `src/cli/` – Command-line argument parsing
- `src/commands/` – Command handlers and external tool integration
- `src/generators/go/` – Go project generation logic
- `docs/` – Documentation and usage guides

## Contributing

Areas needing contribution:
- Git safety checks and wrapper implementation
- Additional framework templates
- Template testing and validation
- Documentation improvements

Run tests before submitting changes:
```bash
cargo fmt
cargo clippy
cargo test
```

## License

MIT
