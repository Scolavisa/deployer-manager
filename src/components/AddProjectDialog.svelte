<script lang="ts">
  import { registerProject, listProjects } from "../lib/api";
  import { projects } from "../lib/stores";

  interface Props {
    open: boolean;
    onClose: () => void;
    onAdded: () => void;
  }

  let { open, onClose, onAdded }: Props = $props();

  let path = $state("");
  let error = $state("");
  let loading = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!path.trim()) {
      error = "Please enter a project path";
      return;
    }

    loading = true;
    error = "";

    try {
      await registerProject(path.trim());
      const updatedProjects = await listProjects();
      projects.set(updatedProjects);
      path = "";
      onAdded();
    } catch (e: any) {
      error = typeof e === "string" ? e : e?.message || "Failed to register project";
    } finally {
      loading = false;
    }
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="backdrop" onclick={handleBackdropClick}>
    <div class="dialog" role="dialog" aria-modal="true" aria-labelledby="dialog-title">
      <div class="dialog-header">
        <h3 id="dialog-title">Add Project</h3>
        <button class="close-btn" onclick={onClose} aria-label="Close">✕</button>
      </div>

      <form onsubmit={handleSubmit}>
        <div class="field">
          <label for="project-path">Project Path</label>
          <input
            id="project-path"
            type="text"
            bind:value={path}
            placeholder="/path/to/project"
            disabled={loading}
          />
          <p class="hint">Path must contain a .deployments/deploy.php file</p>
        </div>

        {#if error}
          <div class="error">{error}</div>
        {/if}

        <div class="actions">
          <button type="button" class="cancel-btn" onclick={onClose} disabled={loading}>
            Cancel
          </button>
          <button type="submit" class="submit-btn" disabled={loading}>
            {loading ? "Adding..." : "Add Project"}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .dialog {
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: 8px;
    width: 420px;
    max-width: 90vw;
    padding: 20px;
  }

  .dialog-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .dialog-header h3 {
    font-size: 1rem;
    font-weight: 600;
  }

  .close-btn {
    background: transparent;
    color: var(--text-muted);
    font-size: 1rem;
    padding: 4px 8px;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .field {
    margin-bottom: 16px;
  }

  .field label {
    display: block;
    font-size: 0.8rem;
    color: var(--text-secondary);
    margin-bottom: 6px;
  }

  .field input {
    width: 100%;
  }

  .hint {
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-top: 4px;
  }

  .error {
    background: rgba(247, 118, 142, 0.1);
    border: 1px solid var(--error);
    border-radius: var(--radius);
    padding: 8px 12px;
    font-size: 0.8rem;
    color: var(--error);
    margin-bottom: 16px;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .cancel-btn {
    background: var(--bg-tertiary);
    color: var(--text-secondary);
  }

  .cancel-btn:hover {
    background: var(--bg-hover);
  }

  .submit-btn {
    background: var(--accent);
    color: var(--bg-primary);
    font-weight: 500;
  }

  .submit-btn:hover:not(:disabled) {
    background: var(--accent-hover);
  }

  .submit-btn:disabled,
  .cancel-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
