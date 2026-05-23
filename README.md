# Deployment Manager

A lightweight desktop application for managing [PHP Deployer](https://deployer.org/) deployments across multiple projects. Built with [Tauri](https://tauri.app/) (Rust backend + Svelte frontend).

## Features

- Register multiple projects and manage deployments from a single interface
- Auto-discovers environments from each project's `hosts.yaml`
- Quick deploy with tag or branch selection
- Real-time streaming of deployment output
- View past release history per environment
- Light and dark theme
- Cross-platform: Linux (primary), macOS (planned)

![example app screen](assets/example_app_screen.png)

## Requirements

- **Rust** (1.70+) — [Install via rustup](https://rustup.rs/)
- **Node.js** (18+) and npm
- **PHP Deployer** (`dep`) installed and available in PATH
- **Git** — for tag/branch discovery
- **System libraries** (Linux):
  - Ubuntu/Debian: `sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev`
  - Fedora: `sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel`
  - Arch: `sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl appmenu-gtk-module libappindicator-gtk3 librsvg`

## Project Structure

```
deployment-manager/
├── src/                    # Svelte frontend
│   ├── components/         # UI components
│   ├── lib/                # API wrappers, stores, event helpers
│   └── types.ts            # TypeScript interfaces
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri IPC command handlers
│   │   ├── services/       # Business logic (config, hosts, git, process)
│   │   └── models/         # Data structures
│   └── Cargo.toml
├── package.json
└── README.md
```

## Development

```bash
# Install frontend dependencies
npm install

# Run in development mode (hot-reload)
npm run tauri dev

# Run Rust tests
cd src-tauri && cargo test

# Type-check the frontend
npm run check
```

## Building

```bash
# Build production binary
npm run tauri build
```

The built binary will be in `src-tauri/target/release/deployment-manager`.
On Linux, a `.deb` package is also generated in `src-tauri/target/release/bundle/deb/`.

## Project Setup

Each project you register must have a `.deployments/` directory containing:

- **`deploy.php`** — PHP Deployer recipe (required for registration)
- **`hosts.yaml`** — Host configuration defining environments

### hosts.yaml format

The app supports two formats:

**Standard PHP Deployer format (with `hosts:` wrapper):**

```yaml
hosts:
  prod:
    hostname: server.example.com
    remote_user: deploy
    deploy_path: /var/www/app
    branch: master
    keep_releases: 5
    stage: prod
  staging:
    hostname: server.example.com
    remote_user: deploy
    deploy_path: /var/www/staging
    branch: develop
    keep_releases: 3
    stage: staging
```

**Flat format (environments at top level):**

```yaml
prod:
  hostname: server.example.com
  remote_user: deploy
  deploy_path: /var/www/app
  keep_releases: 5
staging:
  hostname: server.example.com
  remote_user: deploy
  deploy_path: /var/www/staging
  keep_releases: 3
```

### hosts.yaml fields

| Field | Required | Description |
|-------|----------|-------------|
| `hostname` | yes | Server hostname or IP |
| `remote_user` | yes | SSH user for deployment |
| `deploy_path` | yes | Remote path for deployments |
| `branch` | no | Default branch for this environment |
| `stage` | no | Stage identifier (e.g., "prod", "staging") |
| `keep_releases` | no | Number of recent releases to display in the app (default: 5) |

## Usage

1. **Add a project**: Click "+ Add Project" in the sidebar and enter the path to your project root
2. **Select a project**: Click on it in the sidebar to see its environments
3. **Deploy**: Click "Deploy" on an environment card, select a tag or branch, and hit Deploy
4. **View output**: Deployment output streams in real-time below the environment card
5. **Release history**: Scroll down to see past releases for each environment
6. **Theme**: Click the ☀/🌙 button in the bottom-right corner to toggle light/dark mode

## CLI Commands Used

The app executes these `dep` commands under the hood:

```bash
# Deploy to an environment
dep deploy -f .deployments/deploy.php <environment>
dep deploy -f .deployments/deploy.php <environment> --tag=<tag>
dep deploy -f .deployments/deploy.php <environment> --branch=<branch>

# Fetch release history
dep -f .deployments/deploy.php releases <environment>
```

## Logging

Logs are written to the platform log directory:
- **Linux**: `~/.local/share/com.deployment-manager.app/logs/`

Logs include info about environment discovery, deployment commands executed, and errors.

## Configuration

App configuration (registered projects) is stored at:
- **Linux**: `~/.config/deployment-manager/config.json`
- **macOS**: `~/Library/Application Support/deployment-manager/config.json`

### UI Preferences

UI preferences (like theme choice) are stored in the WebView's `localStorage`, which persists in:
- **Linux**: `~/.local/share/com.deployment-manager.app/` (WebKit data directory)

This means preferences survive app restarts but are tied to the WebView storage, not the config file.

## License

MIT
