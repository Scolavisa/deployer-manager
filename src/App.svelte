<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { listProjects } from "./lib/api";
  import { projects, deploymentOutputs, deploymentStatuses } from "./lib/stores";
  import { onDeployOutput, onDeployComplete } from "./lib/events";
  import ProjectList from "./components/ProjectList.svelte";
  import ProjectPanel from "./components/ProjectPanel.svelte";
  import type { DeploymentOutput, DeployCompleteEvent } from "./types";

  let unlistenOutput: (() => void) | null = null;
  let unlistenComplete: (() => void) | null = null;
  let theme = $state<"dark" | "light">(
    (localStorage.getItem("theme") as "dark" | "light") || "dark"
  );

  function toggleTheme() {
    theme = theme === "dark" ? "light" : "dark";
    localStorage.setItem("theme", theme);
    document.documentElement.setAttribute("data-theme", theme);
  }

  onMount(async () => {
    document.documentElement.setAttribute("data-theme", theme);

    try {
      const projectList = await listProjects();
      projects.set(projectList);
    } catch (e) {
      console.error("Failed to load projects:", e);
    }

    unlistenOutput = await onDeployOutput((output: DeploymentOutput) => {
      deploymentOutputs.update((map) => {
        const existing = map.get(output.deployment_id) || [];
        existing.push(output);
        map.set(output.deployment_id, existing);
        return new Map(map);
      });
    });

    unlistenComplete = await onDeployComplete((event: DeployCompleteEvent) => {
      deploymentStatuses.update((map) => {
        map.set(event.deployment_id, {
          type: "Completed",
          deployment_id: event.deployment_id,
          success: event.success,
          exit_code: event.exit_code,
        });
        return new Map(map);
      });
    });
  });

  onDestroy(() => {
    if (unlistenOutput) unlistenOutput();
    if (unlistenComplete) unlistenComplete();
  });
</script>

<div class="app">
  <aside class="sidebar">
    <ProjectList />
  </aside>
  <main class="content">
    <ProjectPanel />
  </main>
  <button class="theme-toggle" onclick={toggleTheme} title="Toggle theme">
    {theme === "dark" ? "☀" : "🌙"}
  </button>
</div>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(:root),
  :global([data-theme="dark"]) {
    --bg-primary: #1a1b26;
    --bg-secondary: #24283b;
    --bg-tertiary: #2f3347;
    --bg-hover: #363b54;
    --text-primary: #c0caf5;
    --text-secondary: #a9b1d6;
    --text-muted: #565f89;
    --accent: #7aa2f7;
    --accent-hover: #89b4fa;
    --success: #9ece6a;
    --error: #f7768e;
    --warning: #e0af68;
    --border: #3b4261;
    --font-mono: "JetBrains Mono", "Fira Code", "Cascadia Code", monospace;
    --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    --radius: 6px;
    --terminal-bg: #0f0f14;
    --terminal-text: #c0caf5;
  }

  :global([data-theme="light"]) {
    --bg-primary: #f5f5f5;
    --bg-secondary: #ffffff;
    --bg-tertiary: #e8e8e8;
    --bg-hover: #dcdcdc;
    --text-primary: #1a1a2e;
    --text-secondary: #3d3d5c;
    --text-muted: #8888aa;
    --accent: #2563eb;
    --accent-hover: #1d4ed8;
    --success: #16a34a;
    --error: #dc2626;
    --warning: #d97706;
    --border: #d1d5db;
    --terminal-bg: #1e1e2e;
    --terminal-text: #e0e0e0;
  }

  :global(body) {
    font-family: var(--font-sans);
    background: var(--bg-primary);
    color: var(--text-primary);
    overflow: hidden;
    height: 100vh;
  }

  :global(#app) {
    height: 100vh;
  }

  :global(button) {
    font-family: var(--font-sans);
    cursor: pointer;
    border: none;
    border-radius: var(--radius);
    padding: 6px 12px;
    font-size: 0.8rem;
    transition: background 0.15s;
  }

  :global(input, select) {
    font-family: var(--font-sans);
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    color: var(--text-primary);
    padding: 8px 10px;
    font-size: 0.85rem;
    outline: none;
    transition: border-color 0.15s;
  }

  :global(input:focus, select:focus) {
    border-color: var(--accent);
  }

  .app {
    display: flex;
    height: 100vh;
  }

  .sidebar {
    width: 240px;
    min-width: 240px;
    background: var(--bg-secondary);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 20px;
  }

  .theme-toggle {
    position: fixed;
    bottom: 12px;
    right: 12px;
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: var(--accent);
    color: var(--bg-primary);
    border: none;
    font-size: 1.1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    z-index: 50;
    padding: 0;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }

  .theme-toggle:hover {
    background: var(--accent-hover);
  }
</style>
