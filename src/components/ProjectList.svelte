<script lang="ts">
  import { projects, selectedProjectId } from "../lib/stores";
  import type { Project } from "../types";
  import AddProjectDialog from "./AddProjectDialog.svelte";

  let showAddDialog = $state(false);
  let projectList: Project[] = $state([]);
  let currentSelectedId: string | null = $state(null);

  projects.subscribe((value) => {
    projectList = value;
  });

  selectedProjectId.subscribe((value) => {
    currentSelectedId = value;
  });

  function selectProject(project: Project) {
    if (project.available) {
      selectedProjectId.set(project.id);
    }
  }

  function handleAdded() {
    showAddDialog = false;
  }

  function handleClose() {
    showAddDialog = false;
  }
</script>

<div class="project-list">
  <div class="header">
    <h2>Projects</h2>
  </div>

  <div class="list">
    {#each projectList as project (project.id)}
      <button
        class="project-item"
        class:selected={currentSelectedId === project.id}
        class:unavailable={!project.available}
        onclick={() => selectProject(project)}
        disabled={!project.available}
      >
        <span class="project-name">{project.name}</span>
        {#if !project.available}
          <span class="unavailable-icon" title="Path not found">⚠</span>
        {/if}
      </button>
    {/each}

    {#if projectList.length === 0}
      <p class="empty">No projects registered</p>
    {/if}
  </div>

  <div class="footer">
    <button class="add-btn" onclick={() => (showAddDialog = true)}>
      + Add Project
    </button>
  </div>
</div>

{#if showAddDialog}
  <AddProjectDialog open={showAddDialog} onClose={handleClose} onAdded={handleAdded} />
{/if}

<style>
  .project-list {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .header {
    padding: 16px;
    border-bottom: 1px solid var(--border);
  }

  .header h2 {
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .list {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }

  .project-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 10px 12px;
    background: transparent;
    color: var(--text-primary);
    border-radius: var(--radius);
    text-align: left;
    font-size: 0.85rem;
    margin-bottom: 2px;
  }

  .project-item:hover:not(:disabled) {
    background: var(--bg-hover);
  }

  .project-item.selected {
    background: var(--bg-tertiary);
    color: var(--accent);
  }

  .project-item.unavailable {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .project-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .unavailable-icon {
    font-size: 0.75rem;
    color: var(--warning);
    flex-shrink: 0;
    margin-left: 8px;
  }

  .empty {
    padding: 16px 12px;
    color: var(--text-muted);
    font-size: 0.8rem;
    text-align: center;
  }

  .footer {
    padding: 12px;
    border-top: 1px solid var(--border);
  }

  .add-btn {
    width: 100%;
    padding: 8px;
    background: var(--accent);
    color: var(--bg-primary);
    font-weight: 500;
    font-size: 0.8rem;
  }

  .add-btn:hover {
    background: var(--accent-hover);
  }
</style>
