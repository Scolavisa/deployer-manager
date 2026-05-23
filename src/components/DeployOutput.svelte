<script lang="ts">
  import { deploymentOutputs, deploymentStatuses } from "../lib/stores";
  import type { DeploymentOutput, DeploymentStatus } from "../types";

  interface Props {
    deploymentId: string;
    onCompleted?: () => void;
  }

  let { deploymentId, onCompleted }: Props = $props();

  let outputLines: DeploymentOutput[] = $state([]);
  let status: DeploymentStatus | undefined = $state(undefined);
  let container: HTMLDivElement | undefined = $state(undefined);
  let autoScroll = $state(true);

  deploymentOutputs.subscribe((map) => {
    outputLines = map.get(deploymentId) || [];
    if (autoScroll && container) {
      requestAnimationFrame(() => {
        if (container) {
          container.scrollTop = container.scrollHeight;
        }
      });
    }
  });

  deploymentStatuses.subscribe((map) => {
    const newStatus = map.get(deploymentId);
    if (newStatus?.type === "Completed" && status?.type !== "Completed") {
      if (onCompleted) {
        onCompleted();
      }
    }
    status = newStatus;
  });

  function handleScroll() {
    if (!container) return;
    const { scrollTop, scrollHeight, clientHeight } = container;
    autoScroll = scrollHeight - scrollTop - clientHeight < 30;
  }
</script>

<div class="deploy-output">
  {#if status?.type === "Completed"}
    <div class="status-bar" class:success={status.success} class:failure={!status.success}>
      {#if status.success}
        ✓ Deployment completed successfully
      {:else}
        ✗ Deployment failed (exit code: {status.exit_code})
      {/if}
    </div>
  {/if}

  <div class="terminal" bind:this={container} onscroll={handleScroll}>
    {#each outputLines as line}
      <div class="line" class:stderr={line.stream === "Stderr"}>
        {line.line}
      </div>
    {/each}
    {#if outputLines.length === 0 && !status}
      <div class="line muted">Deploying...</div>
    {/if}
  </div>
</div>

<style>
  .deploy-output {
    margin-top: 10px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
  }

  .status-bar {
    padding: 6px 12px;
    font-size: 0.78rem;
    font-weight: 500;
  }

  .status-bar.success {
    background: rgba(158, 206, 106, 0.15);
    color: var(--success);
    border-bottom: 1px solid rgba(158, 206, 106, 0.3);
  }

  .status-bar.failure {
    background: rgba(247, 118, 142, 0.15);
    color: var(--error);
    border-bottom: 1px solid rgba(247, 118, 142, 0.3);
  }

  .terminal {
    background: var(--terminal-bg, #0f0f14);
    padding: 10px;
    max-height: 300px;
    overflow-y: auto;
    font-family: var(--font-mono);
    font-size: 0.75rem;
    line-height: 1.5;
  }

  .line {
    white-space: pre-wrap;
    word-break: break-all;
    color: var(--terminal-text, var(--text-primary));
  }

  .line.stderr {
    color: var(--error);
  }

  .line.muted {
    color: var(--text-muted);
    font-style: italic;
  }
</style>
