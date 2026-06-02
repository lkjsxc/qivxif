<script lang="ts">
  let { state: viewState } = $props();

  const paneCount = $derived(countPanes(viewState.layout?.root));

  function countPanes(tile) {
    if (!tile) return 0;
    if (tile.kind === "stack") return tile.tabs.length;
    return (tile.children ?? []).reduce((sum, child) => sum + countPanes(child), 0);
  }
</script>

<section class="tab-panel diagnostics">
  <h1>Diagnostics</h1>
  <p>Service worker: {viewState.serviceWorkerReady ? "ready" : "pending"}</p>
  <p class="mono">Last error: {viewState.lastError || "none"}</p>
  <p>Layout panes: {paneCount}</p>
  <p>Queued: {viewState.queued} · Rejected: {viewState.rejected}</p>
</section>
