<script lang="ts">
  import { selectedProjectId, projects } from "../lib/stores";
  import { getEnvironments, removeProject, listProjects } from "../lib/api";
  import type { Project, Environment } from "../types";
  import EnvironmentCard from "./EnvironmentCard.svelte";
  import ReleaseHistory from "./ReleaseHistory.svelte";

  let currentProjectId: string | null = $state(null);
  let project: Project | undefined = $state(undefined);
  let environments: Environment[] = $state([]);
  let loadingEnvs = $state(false);
  let envError = $state("");
  let confirmRemove = $state(false);
  let selectedEnvForHistory: string = $state("");
  let releaseRefreshKey = $state(0);

  function handleDeployCompleted() {
    // Increment key to force ReleaseHistory to re-fetch
    releaseRefreshKey++;
  }

  selectedProjectId.subscribe((id) => {
    currentProjectId = id;
    confirmRemove = false;
    selectedEnvForHistory = "";
  });

  projects.subscribe((list) => {
    if (currentProjectId) {
      project = list.find((p) => p.id === currentProjectId);
    } else {
      project = undefined;
    }
  });

  $effect(() => {
    if (currentProjectId) {
      project = undefined;
      projects.subscribe((list) => {
        project = list.find((p) => p.id === currentProjectId);
      })();
      fetchEnvironments(currentProjectId);
    } else {
      environments = [];
      project = undefined;
    }
  });

  async function fetchEnvironments(projectId: string) {
    loadingEnvs = true;
    envError = "";
    try {
      environments = await getEnvironments(projectId);
      if (environments.length > 0 && !selectedEnvForHistory) {
        selectedEnvForHistory = environments[0].name;
      }
    } catch (e: any) {
      envError = typeof e === "string" ? e : "Failed to load environments";
      environments = [];
    } finally {
      loadingEnvs = false;
    }
  }

  async function handleRemove() {
    if (!currentProjectId) return;
    try {
      await removeProject(currentProjectId);
      const updatedProjects = await listProjects();
      projects.set(updatedProjects);
      selectedProjectId.set(null);
    } catch (e: any) {
      console.error("Failed to remove project:", e);
    }
  }
</script>

{#if !currentProjectId}
  <div class="empty-state">
    <p>Select a project from the sidebar to get started</p>
  </div>
{:else if project}
  <div class="project-panel">
    <div class="panel-header">
      <div>
        <h2>{project.name}</h2>
        <p class="project-path">{project.path}</p>
      </div>
      <div class="header-actions">
        {#if confirmRemove}
          <span class="confirm-text">Remove this project?</span>
          <button class="confirm-yes" onclick={handleRemove}>Yes, remove</button>
          <button class="confirm-no" onclick={() => (confirmRemove = false)}>Cancel</button>
        {:else}
          <button class="remove-btn" onclick={() => (confirmRemove = true)}>Remove</button>
        {/if}
      </div>
    </div>

    <section class="environments-section">
      <h3>Environments</h3>
      {#if loadingEnvs}
        <p class="muted">Loading environments...</p>
      {:else if envError}
        <p class="error">{envError}</p>
      {:else if environments.length === 0}
        <p class="muted">No environments found in hosts.yaml</p>
      {:else}
        {#each environments as env (env.name)}
          <EnvironmentCard environment={env} projectId={project.id} onDeployCompleted={handleDeployCompleted} />
        {/each}
      {/if}
    </section>

    {#if environments.length > 0 && currentProjectId}
      <section class="history-section">
        <div class="history-header">
          <h3>Release History</h3>
          {#if environments.length > 1}
            <select
              class="env-select"
              value={selectedEnvForHistory}
              onchange={(e) => (selectedEnvForHistory = (e.target as HTMLSelectElement).value)}
            >
              {#each environments as env}
                <option value={env.name}>{env.name}</option>
              {/each}
            </select>
          {/if}
        </div>
        {#if selectedEnvForHistory}
          {@const selectedEnv = environments.find(e => e.name === selectedEnvForHistory)}
          {#key releaseRefreshKey}
            <ReleaseHistory projectId={currentProjectId} environment={selectedEnvForHistory} keepReleases={selectedEnv?.keep_releases} />
          {/key}
        {/if}
      </section>
    {/if}
  </div>
{/if}

<style>
  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-muted);
    font-size: 0.9rem;
  }

  .project-panel {
    max-width: 800px;
  }

  .panel-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    margin-bottom: 24px;
    gap: 16px;
  }

  .panel-header h2 {
    font-size: 1.2rem;
    font-weight: 600;
    margin-bottom: 4px;
  }

  .project-path {
    font-size: 0.78rem;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
  }

  .confirm-text {
    font-size: 0.78rem;
    color: var(--warning);
  }

  .confirm-yes {
    background: var(--error);
    color: white;
    font-size: 0.75rem;
  }

  .confirm-yes:hover {
    background: #ff8fa3;
  }

  .confirm-no {
    background: var(--bg-tertiary);
    color: var(--text-secondary);
    font-size: 0.75rem;
  }

  .confirm-no:hover {
    background: var(--bg-hover);
  }

  .remove-btn {
    background: var(--bg-tertiary);
    color: var(--error);
    font-size: 0.78rem;
  }

  .remove-btn:hover {
    background: rgba(247, 118, 142, 0.15);
  }

  .environments-section,
  .history-section {
    margin-bottom: 24px;
  }

  .environments-section h3,
  .history-section h3 {
    font-size: 0.88rem;
    font-weight: 600;
    margin-bottom: 12px;
    color: var(--text-primary);
  }

  .history-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
  }

  .env-select {
    font-size: 0.78rem;
    padding: 4px 8px;
  }

  .muted {
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  .error {
    font-size: 0.8rem;
    color: var(--error);
  }
</style>
