# Contributing

Thanks for your interest in contributing to Deployment Manager. This document covers the technical setup, architecture, and guidelines for contributors.

## Development Setup

### Prerequisites

- **Rust** 1.70+ — [Install via rustup](https://rustup.rs/)
- **Node.js** 18+ and npm
- **PHP Deployer** (`dep`) in PATH (for testing deployments)
- **Git**

### System Libraries (Linux)

Ubuntu/Debian:
```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

Fedora:
```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel
```

Arch:
```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl appmenu-gtk-module libappindicator-gtk3 librsvg
```

### Getting Started

```bash
git clone <repo-url>
cd deployment-manager

# Install frontend dependencies
npm install

# Run in development mode (hot-reload for frontend, rebuilds Rust on change)
npm run tauri dev

# Run all Rust tests (unit + integration + property-based)
cd src-tauri && cargo test

# Type-check the frontend
npm run check

# Build production binary
npm run tauri build
```

## Architecture

The app is built with [Tauri 2.x](https://tauri.app/) — a Rust backend with a web-based frontend.

```
deployment-manager/
├── src/                        # Frontend (Svelte 5 + TypeScript)
│   ├── components/             # UI components
│   │   ├── App.svelte          # Root layout, event listeners, theme
│   │   ├── ProjectList.svelte  # Sidebar project list
│   │   ├── ProjectPanel.svelte # Main content area
│   │   ├── EnvironmentCard.svelte
│   │   ├── DeployForm.svelte   # Tag/branch selection
│   │   ├── DeployOutput.svelte # Terminal output display
│   │   ├── ReleaseHistory.svelte
│   │   └── AddProjectDialog.svelte
│   ├── lib/
│   │   ├── api.ts              # Typed wrappers around Tauri invoke()
│   │   ├── events.ts           # Tauri event listeners
│   │   └── stores.ts           # Svelte writable stores
│   └── types.ts                # TypeScript interfaces
├── src-tauri/                  # Backend (Rust)
│   ├── src/
│   │   ├── main.rs             # Entry point
│   │   ├── lib.rs              # Tauri app setup, command registration
│   │   ├── state.rs            # Shared AppState (Mutex-wrapped)
│   │   ├── error.rs            # AppError enum
│   │   ├── commands/           # Tauri IPC command handlers
│   │   │   ├── projects.rs     # register, remove, list, get
│   │   │   ├── environments.rs # get_environments
│   │   │   ├── deployments.rs  # start_deployment, get_status
│   │   │   ├── releases.rs     # get_releases + output parser
│   │   │   └── git.rs          # get_tags, get_branches
│   │   ├── services/           # Business logic
│   │   │   ├── config.rs       # ConfigManager (load/save JSON)
│   │   │   ├── hosts.rs        # HostsParser (YAML → Environment)
│   │   │   ├── project.rs      # Project validation & management
│   │   │   ├── process.rs      # Subprocess execution & streaming
│   │   │   └── git.rs          # Git tag/branch retrieval
│   │   └── models/             # Data structures (serde)
│   │       ├── config.rs       # AppConfig, ProjectConfig
│   │       ├── project.rs      # Project
│   │       ├── environment.rs  # Environment
│   │       ├── deployment.rs   # DeploymentStatus, DeploymentOutput
│   │       └── release.rs      # Release
│   ├── tests/
│   │   └── integration_tests.rs
│   └── Cargo.toml
├── package.json
├── vite.config.ts
└── tsconfig.json
```

### Communication Flow

1. Frontend calls backend via `invoke("command_name", { params })` (Tauri IPC)
2. Backend processes the request (file I/O, subprocess execution, etc.)
3. For deployments: backend streams output via Tauri events (`deploy_output`, `deploy_complete`)
4. Frontend listens to events and updates Svelte stores reactively

### Key Design Decisions

- **No framework** on the frontend — Svelte 5 with runes, no router needed for a single-view app
- **Mutex-based state** — simple and sufficient for a desktop app with no concurrent users
- **Subprocess execution** — shells out to `dep` and `git` CLI tools rather than reimplementing their logic
- **YAML parsing in Rust** — keeps file system concerns in the backend
- **Alphabetical environment ordering** — parsed from HashMap, sorted for consistent display

## Testing

### Running Tests

```bash
cd src-tauri

# All tests
cargo test

# Only unit tests (faster, no integration)
cargo test --lib

# Specific module
cargo test --lib services::hosts

# With output
cargo test -- --nocapture
```

### Test Structure

- **Unit tests** — inline `#[cfg(test)] mod tests` in each module
- **Property-based tests** — using `proptest` crate, verify universal properties with generated inputs
- **Integration tests** — in `tests/integration_tests.rs`, test full flows (config persistence, project registration)

### Property-Based Tests

We use `proptest` to verify correctness properties. Each property test is tagged with a comment:
```rust
// Feature: deployment-manager, Property N: <description>
// **Validates: Requirements X.Y**
```

Properties covered:
1. Project validation matches file existence
2. Project removal decreases list by one
3. Configuration serialization round-trip
4. Path availability reflects file system state
5. Hosts parsing extracts all environments
6. Invalid YAML produces parse error
7. Deploy command construction
8. Exit code determines deployment success
9. Releases command construction
10. Releases sorted reverse chronologically
11. Tag and branch are mutually exclusive

## Code Style & Guidelines

### General

- Keep it simple. This is a utility app, not a framework.
- No unnecessary abstractions. A function is fine; not everything needs a trait.
- Error messages should be actionable — tell the user what went wrong and what to check.

### Rust

- Use `thiserror` for error types, `serde` for serialization
- All commands return `Result<T, AppError>` — errors serialize to the frontend
- Use `log::info!` / `log::error!` for operations that help debugging (file paths, command args, exit codes)
- Run `cargo clippy` before submitting — no warnings allowed
- Run `cargo fmt` — consistent formatting

### Frontend (Svelte/TypeScript)

- Svelte 5 runes syntax (`$state`, `$derived`, `$effect`, `$props`)
- All components use `<script lang="ts">`
- Types in `src/types.ts` must match Rust struct serialization
- API calls go through `src/lib/api.ts` — never call `invoke()` directly from components
- Scoped `<style>` in components — no global CSS except in `App.svelte`

### Commits

- Use conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `test:`, `chore:`
- Keep commits focused — one logical change per commit
- Write a clear commit message explaining *why*, not just *what*

### Pull Requests

- One feature or fix per PR
- Include tests for new functionality
- Update README.md if the change affects user-facing behavior
- Property-based tests are encouraged for any new validation logic
- All tests must pass (`cargo test` + `npm run check`)

## Adding a New Feature

Typical flow:

1. Add/update the Rust model in `src-tauri/src/models/`
2. Implement the service logic in `src-tauri/src/services/`
3. Create the Tauri command in `src-tauri/src/commands/`
4. Register the command in `src-tauri/src/lib.rs`
5. Add the TypeScript interface in `src/types.ts`
6. Add the API function in `src/lib/api.ts`
7. Build/update the Svelte component
8. Write tests (unit + property if applicable)

## Known Limitations

- **No real-time output streaming** — PHP Deployer buffers stdout when not connected to a TTY. Output appears after the deployment completes. This is a known limitation we'd like to solve.
- **Linux only** — macOS support is planned but not yet tested.
- **No concurrent deployments** — deploying to the same environment while one is running is not prevented at the UI level (the backend handles it gracefully).

## License

MIT — see [LICENCE](LICENCE)
