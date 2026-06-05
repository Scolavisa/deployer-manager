<script lang="ts">
  import { getTags, getBranches, startDeployment } from "../lib/api";

  interface Props {
    projectId: string;
    environment: string;
    onClose: () => void;
    onDeployStarted: (deploymentId: string) => void;
    onDeployingChange: (deploying: boolean) => void;
  }

  let { projectId, environment, onClose, onDeployStarted, onDeployingChange }: Props = $props();

  let tags: string[] = $state([]);
  let branches: string[] = $state([]);
  let selectedTag = $state("");
  let selectedBranch = $state("");
  let loading = $state(true);
  let deploying = $state(false);
  let error = $state("");

  $effect(() => {
    loadGitData();
  });

  async function loadGitData() {
    loading = true;
    error = "";
    try {
      const [fetchedTags, fetchedBranches] = await Promise.all([
        getTags(projectId),
        getBranches(projectId),
      ]);
      tags = fetchedTags;
      branches = fetchedBranches;
    } catch (e: any) {
      error = typeof e === "string" ? e : "Failed to load git data";
    } finally {
      loading = false;
    }
  }

  function handleTagChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    selectedTag = target.value;
    if (selectedTag) {
      selectedBranch = "";
    }
  }

  function handleBranchChange(e: Event) {
    const target = e.target as HTMLSelectElement;
    selectedBranch = target.value;
    if (selectedBranch) {
      selectedTag = "";
    }
  }

  async function handleDeploy() {
    if (!selectedTag && !selectedBranch) {
      error = "Please select a tag or branch";
      return;
    }

    deploying = true;
    onDeployingChange(true);
    error = "";

    try {
      const deploymentId = await startDeployment(
        projectId,
        environment,
        selectedTag || undefined,
        selectedBranch || undefined
      );
      onDeployingChange(false);
      onDeployStarted(deploymentId);
    } catch (e: any) {
      error = typeof e === "string" ? e : "Failed to start deployment";
      deploying = false;
      onDeployingChange(false);
    }
  }
</script>

<div class="deploy-form">
  <div class="form-header">
    <h5>Deploy to {environment}</h5>
    <button class="close-btn" onclick={onClose} aria-label="Close">✕</button>
  </div>

  {#if loading}
    <div class="loading">Loading tags and branches...</div>
  {:else}
    <div class="form-fields">
      <div class="field">
        <label for="tag-select">Tag</label>
        <select
          id="tag-select"
          value={selectedTag}
          onchange={handleTagChange}
          disabled={deploying}
        >
          <option value="">— Select tag —</option>
          {#each tags as tag}
            <option value={tag}>{tag}</option>
          {/each}
        </select>
      </div>

      <div class="separator">or</div>

      <div class="field">
        <label for="branch-select">Branch</label>
        <select
          id="branch-select"
          value={selectedBranch}
          onchange={handleBranchChange}
          disabled={deploying}
        >
          <option value="">— Select branch —</option>
          {#each branches as branch}
            <option value={branch}>{branch}</option>
          {/each}
        </select>
      </div>
    </div>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    <div class="form-actions">
      <button class="cancel-btn" onclick={onClose} disabled={deploying}>Cancel</button>
      <button class="deploy-btn" onclick={handleDeploy} disabled={deploying}>
        {#if deploying}
          <span class="spinner" aria-hidden="true">↻</span>
          Deploying...
        {:else}
          Deploy
        {/if}
      </button>
    </div>
  {/if}
</div>

<style>
  .deploy-form {
    margin-top: 12px;
    padding: 12px;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--radius);
  }

  .form-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 12px;
  }

  .form-header h5 {
    font-size: 0.82rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .close-btn {
    background: transparent;
    color: var(--text-muted);
    font-size: 0.85rem;
    padding: 2px 6px;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .loading {
    font-size: 0.8rem;
    color: var(--text-muted);
    padding: 8px 0;
  }

  .form-fields {
    display: flex;
    align-items: flex-end;
    gap: 10px;
    margin-bottom: 12px;
  }

  .field {
    flex: 1;
  }

  .field label {
    display: block;
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-bottom: 4px;
  }

  .field select {
    width: 100%;
  }

  .separator {
    font-size: 0.75rem;
    color: var(--text-muted);
    padding-bottom: 8px;
  }

  .error {
    font-size: 0.78rem;
    color: var(--error);
    margin-bottom: 10px;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .cancel-btn {
    background: var(--bg-tertiary);
    color: var(--text-secondary);
  }

  .cancel-btn:hover:not(:disabled) {
    background: var(--bg-hover);
  }

  .deploy-btn {
    background: var(--success);
    color: var(--bg-primary);
    font-weight: 500;
    display: inline-flex;
    align-items: center;
    gap: 6px;
  }

  .deploy-btn:hover:not(:disabled) {
    background: #a9d87a;
  }

  .deploy-btn:disabled,
  .cancel-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .spinner {
    display: inline-block;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }
</style>
