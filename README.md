# xg — Safe Git with Supercharged Project Starters

xg (pronounced **ex-gee**) is an opinionated developer tool that will eventually give Git new safety rails and bundle a powerful project templating system. Think of it as Git’s modern cousin: familiar commands, but with smart defaults and batteries included.

> **Current focus**: building the Go project templating flow before layering in the Git wrapper and release tooling.

---

## Why xg?

- **Safe-by-default Git** – catch `.env`, build artifacts, or IDE clutter before they sneak into a push.
- **Language-aware templates** – spin up ready-to-ship projects with framework-specific structure, configs, and docs.
- **Zero-friction adoption** – alias `git` to `xg` when you want the guard rails, and fall back to plain Git whenever you need.

---

## Status: Pre-Alpha

| Track | What exists today | What’s next |
| --- | --- | --- |
| Project generator | `xg init` CLI skeleton + Go templates directory | Plug generator logic into CLI, add prompts, write tests |
| Git experience | Planned | Pass-through shim with safety checks after Go templates land |
| Releases | Planned | Package binaries + docs after Git wrapper milestone |

If you want to try the current CLI skeleton anyway:

```bash
cargo run -- init \
  --lang go \
  --project awesome-api \
  --rest-framework gin
```

Right now this only echoes your choices; real file generation will be wired up as part of the first milestone.

---

## Go Template Milestone (In Progress)

Goal: generate production-ready Go REST APIs out of the box, starting with **Gin** and **Echo**:

- Scaffold idiomatic folder layout (cmd/, internal/, pkg/, configs/)
- Drop framework-specific bootstrap code and health handlers
- Write `.gitignore`, `README`, Makefile, and Docker bits
- Offer optional components (PostgreSQL, Redis, OpenAPI, CI workflow)
- Back templates with tests to ensure regenerated projects compile

Deliverables ship behind `xg init` so that is the first command worth adopting.

---

## Upcoming Roadmap

1. **Go Project Template Generation** – interactive prompts, template registry, and generator engine (current milestone).
2. **Git Wrapper & Safety Layer** – pass every git command through xg, add staged file scanner, and block risky pushes by default.
3. **First Release** – package binaries, publish install docs, and cut v0.1.0 once Go templates + Git safety are stable.
4. **Other App Templates** – expand into Rust, JavaScript/TypeScript, Python, etc., after the first release proves the workflow.

See `Planning.md` for the full execution breakdown.

---

## Development Setup

```bash
git clone https://github.com/<you>/xg
cd xg
cargo run -- init --help
```

Useful directories:

- `src/cli` – clap-powered argument parsing and subcommand routing
- `src/commands` – handlers for each command (`init` today)
- `templates/go/*` – language/framework blueprints under active development
- `docs/` – supporting documentation (usage, templates, FAQ, etc.)

---

## Contributing

Contributions are welcome even this early. Good first issues:

- Add Go template files (see `templates/go`)
- Flesh out generator logic in `src/generators`
- Document template conventions in `docs/TEMPLATES.md`

Before opening a PR, skim [CONTRIBUTING.md](CONTRIBUTING.md) and make sure `cargo fmt && cargo clippy && cargo test` all pass.

---

## License

MIT © [Nishant](https://github.com/nishujangra)
