<script lang="ts">
  let { state: viewState } = $props();

  const paneCount = $derived(countPanes(viewState.layout?.root));
  const storage = $derived(viewState.storageStatus);
  const stores = $derived(Object.entries(storage?.stores ?? {}).sort());

  function countPanes(tile) {
    if (!tile) return 0;
    if (tile.kind === "stack") return tile.tabs.length;
    return (tile.children ?? []).reduce((sum, child) => sum + countPanes(child), 0);
  }

  function bytes(value) {
    return `${Math.round(Number(value ?? 0) / 1024)} KiB`;
  }
</script>

<section class="tab-panel diagnostics">
  <h1>Diagnostics</h1>
  <p>Service worker: {viewState.serviceWorkerReady ? "ready" : "pending"}</p>
  <p>Service reachability: {viewState.online ? "online" : "offline"}</p>
  <p>Storage mode: {storage?.mode ?? "unknown"}</p>
  {#if storage?.reason}<p>Storage detail: {storage.reason}</p>{/if}
  <p>SQLite pages: {storage?.pageCount ?? "unavailable"}</p>
  <p>Quota: {bytes(storage?.usage)} / {bytes(storage?.quota)}</p>
  <p>Queue: dirty {storage?.queue?.dirty ?? viewState.queued} · pending {storage?.queue?.pending ?? 0} · rejected {storage?.queue?.rejected ?? viewState.rejected} · accepted {storage?.queue?.accepted ?? viewState.acceptedCount}</p>
  <p>Cache: protected {bytes(storage?.cache?.protected)} · prunable {bytes(storage?.cache?.prunable)}</p>
  <p class="mono">Last storage error: {storage?.lastOperationError || "none"}</p>
  <p class="mono">Last shell error: {viewState.lastError || "none"}</p>
  <p>Layout panes: {paneCount}</p>
  {#if stores.length}
    <details>
      <summary>Inventory by repository</summary>
      <dl class="settings-inventory">
        {#each stores as [name, count]}
          <div><dt>{name}</dt><dd>{count}</dd></div>
        {/each}
      </dl>
    </details>
  {/if}
</section>
