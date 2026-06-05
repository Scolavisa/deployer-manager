<script lang="ts">
  import type { Environment } from "../types";
  import DeployForm from "./DeployForm.svelte";
  import DeployOutput from "./DeployOutput.svelte";

  interface Props {
    environment: Environment;
    projectId: string;
    onDeployCompleted?: () => void;
  }

  let { environment, projectId, onDeployCompleted }: Props = $props();

  let showDeployForm = $state(false);
  let activeDeploymentId: string | null = $state(null);
  let deploying = $state(false);

  function handleDeployStarted(deploymentId: string) {
    activeDeploymentId = deploymentId;
    deploying = false;
    showDeployForm = false;
  }

  function handleCloseForm() {
    showDeployForm = false;
  }
  function handleDeployingChange(isDeploying: boolean) {
    deploying = isDeploying;
  }

  function handleDeployCompleted() {
    if (onDeployCompleted) {
      onDeployCompleted();
    }
  }
</script>

<div class="env-card">
  <div class="env-header">
    <div class="env-info">
      <h4 class="env-name">{environment.name}</h4>
      <div class="env-details">
        <span class="detail">
          <span class="label">Host:</span> {environment.hostname}
        </span>
        <span class="detail">
          <span class="label">Path:</span> {environment.deploy_path}
        </span>
        {#if environment.branch}
          <span class="detail">
            <span class="label">Branch:</span> {environment.branch}
          </span>
        {/if}
        {#if environment.keep_releases}
          <span class="detail">
            <span class="label">Keep releases:</span> {environment.keep_releases}
          </span>
        {/if}
      </div>
    </div>
    <button class="deploy-btn" onclick={() => (showDeployForm = true)} disabled={deploying}>
      {#if deploying}
        <span class="spinner" aria-hidden="true">↻</span>
        Deploying
      {:else}
        Deploy
      {/if}
    </button>
  </div>

  {#if showDeployForm}
    <DeployForm
      {projectId}
      environment={environment.name}
      onClose={handleCloseForm}
      onDeployStarted={handleDeployStarted}
      onDeployingChange={handleDeployingChange}
    />
  {/if}

  {#key activeDeploymentId}
    {#if activeDeploymentId && !deploying}
      <DeployOutput deploymentId={activeDeploymentId} onCompleted={handleDeployCompleted} />
    {/if}
  {/key}
</div>

<style>
  .env-card {
    background: var(--bg-tertiary);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 14px;
    margin-bottom: 12px;
  }

  .env-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 12px;
  }

  .env-info {
    flex: 1;
    min-width: 0;
  }

  .env-name {
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--accent);
    margin-bottom: 6px;
    text-transform: capitalize;
  }

  .env-details {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .detail {
    font-size: 0.78rem;
    color: var(--text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .detail .label {
    color: var(--text-muted);
  }

  .deploy-btn {
    background: var(--accent);
    color: var(--bg-primary);
    font-weight: 500;
    padding: 6px 14px;
    display: inline-flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .deploy-btn:hover:not(:disabled) {
    background: var(--accent-hover);
  }

  .deploy-btn:disabled {
    opacity: 0.7;
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
