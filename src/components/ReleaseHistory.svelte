<script lang="ts">
  import { getReleases } from "../lib/api";
  import type { Release } from "../types";

  interface Props {
    projectId: string;
    environment: string;
    keepReleases?: number;
  }

  let { projectId, environment, keepReleases }: Props = $props();

  let allReleases: Release[] = $state([]);
  let loading = $state(false);
  let error = $state("");

  // Show only the last N releases (keep_releases from hosts.yaml, default 5)
  let displayLimit = $derived(keepReleases || 5);
  let releases = $derived(allReleases.slice(0, displayLimit));

  $effect(() => {
    if (projectId && environment) {
      fetchReleases();
    }
  });

  async function fetchReleases() {
    loading = true;
    error = "";
    try {
      allReleases = await getReleases(projectId, environment);
    } catch (e: any) {
      error = typeof e === "string" ? e : "Failed to load releases";
      allReleases = [];
    } finally {
      loading = false;
    }
  }
</script>

<div class="release-history">
  <div class="section-header">
    <h5>Releases — {environment}</h5>
    <button class="refresh-btn" onclick={fetchReleases} disabled={loading}>
      {loading ? "..." : "↻ Refresh"}
    </button>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if loading && allReleases.length === 0}
    <p class="muted">Loading releases...</p>
  {:else if allReleases.length === 0}
    <p class="muted">No releases found</p>
  {:else}
    <div class="release-table">
      <div class="table-header">
        <span class="col-release">#</span>
        <span class="col-date">Date</span>
        <span class="col-target">Branch</span>
        <span class="col-status">Status</span>
      </div>
      {#each releases as release}
        <div class="release-row" class:current={release.is_current}>
          <span class="col-release">{release.name}</span>
          <span class="col-date">{release.date || "—"}</span>
          <span class="col-target">{release.target || "—"}</span>
          <span class="col-status">
            {#if release.is_current}
              <span class="badge">current</span>
            {/if}
          </span>
        </div>
      {/each}
    </div>
    {#if allReleases.length > displayLimit}
      <p class="muted showing-info">Showing {displayLimit} of {allReleases.length} releases</p>
    {/if}
  {/if}
</div>

<style>
  .release-history {
    margin-top: 16px;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 10px;
  }

  .section-header h5 {
    font-size: 0.82rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .refresh-btn {
    background: var(--bg-tertiary);
    color: var(--text-secondary);
    font-size: 0.75rem;
    padding: 4px 10px;
  }

  .refresh-btn:hover:not(:disabled) {
    background: var(--bg-hover);
  }

  .refresh-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .error {
    font-size: 0.78rem;
    color: var(--error);
    margin-bottom: 8px;
  }

  .muted {
    font-size: 0.8rem;
    color: var(--text-muted);
  }

  .showing-info {
    margin-top: 8px;
    font-size: 0.72rem;
  }

  .release-table {
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
  }

  .table-header {
    display: flex;
    padding: 8px 12px;
    background: var(--bg-tertiary);
    font-size: 0.72rem;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.04em;
    border-bottom: 1px solid var(--border);
  }

  .release-row {
    display: flex;
    padding: 8px 12px;
    font-size: 0.78rem;
    border-bottom: 1px solid var(--border);
    color: var(--text-secondary);
  }

  .release-row:last-child {
    border-bottom: none;
  }

  .release-row.current {
    background: rgba(122, 162, 247, 0.05);
  }

  .col-release {
    flex: 0.5;
    font-weight: 500;
  }

  .col-date {
    flex: 1.5;
  }

  .col-target {
    flex: 1.5;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .col-status {
    flex: 0.7;
    text-align: right;
  }

  .badge {
    background: var(--accent);
    color: var(--bg-primary);
    font-size: 0.68rem;
    font-weight: 600;
    padding: 2px 6px;
    border-radius: 3px;
    text-transform: uppercase;
  }
</style>
