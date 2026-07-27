---
name: testing-deployments
description: How to run and test the deployer-manager Tauri app locally, including creating a fake project + fake `dep` binary to test deployment output streaming without real servers.
---

# Testing deployer-manager locally

## Run the app
- `npm install` then `npm run tauri dev` (first Rust build ~3-5 min). webkit2gtk-4.1 dev libs are required (see CONTRIBUTING.md); on this box they were already present.
- Maximize the window before recording: `wmctrl -r "Deployment Manager" -b add,maximized_vert,maximized_horz`.
- Tests: `cd src-tauri && cargo test`; type-check: `npm run check`.

## Project registration
- Config lives at `~/.config/deployment-manager/config.json`. Register via UI: "+ Add Project" → type an absolute path (plain text input, no native file dialog).
- A valid project needs `.deployments/deploy.php` (validation) and `.deployments/hosts.yaml` (environments, PHP Deployer format with a top-level `hosts:` key).
- The deploy form's branch dropdown lists **remote** branches (`git branch -r`), so the fixture must be a clone of a repo with at least one pushed branch — a plain `git init` project shows no branches and you cannot start a deployment.

## Fake `dep` for deployment testing (no real servers needed)
`dep` is resolved via `which dep` first, so a symlink in `/usr/local/bin/dep` is picked up even if the app was launched before PATH changes:

```bash
cat > ~/bin/dep <<'EOF'
#!/bin/bash
echo "task deploy:prepare"; sleep 6
echo "task deploy:vendors"; sleep 6
echo "task deploy:symlink"; sleep 6
echo "error: connection to host timed out (IP restricted)" >&2
exit 1
EOF
chmod +x ~/bin/dep && sudo ln -sf ~/bin/dep /usr/local/bin/dep
```

Use sleeps of >= 5s per line: with 1-2s gaps the whole run finishes faster than screenshot round-trips and you cannot capture intermediate states. To prove output is streamed live (not batched at the end), screenshot at intervals and assert the visible line count strictly increases while `pgrep -af bin/dep` still shows the deploy process.

Note: Release History also invokes `dep ... releases`, so a fake `dep` makes "Failed to load releases" appear — harmless, and beware that `pgrep dep` may match the releases process rather than the deployment.

## Devin Secrets Needed
None.
