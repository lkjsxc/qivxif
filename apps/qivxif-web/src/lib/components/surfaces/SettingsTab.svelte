<script lang="ts">
  let { state: viewState, actions } = $props();

  const paneCount = $derived(countPanes(viewState.layout?.root));
  const storage = $derived(viewState.storageStatus);
  const stores = $derived(Object.entries(storage?.stores ?? {}).sort());

  function countPanes(tile) {
    if (!tile) return 0;
    if (tile.kind === "stack") return tile.tabs.length;
    return (tile.children ?? []).reduce((sum, child) => sum + countPanes(child), 0);
  }

  function bytes(value) {
    const size = Number(value ?? 0);
    return `${Math.round(size / 1024)} KiB`;
  }
</script>

<section class="tab-panel settings">
  <h1>Settings</h1>
  <p>{viewState.auth ? `Signed in as ${viewState.auth.user.name}` : "Signed out"}</p>
  <p>Layout panes: {paneCount}</p>
  <p>Maximized: {viewState.layout?.maximized_pane_id ?? "none"}</p>
  <p>Storage mode: {storage?.mode ?? "unknown"}</p>
  {#if storage?.mode === "memory"}<p>Storage is degraded; reload may lose local changes.</p>{/if}
  {#if storage?.mode === "unavailable"}<p>Local storage is unavailable.</p>{/if}
  {#if storage?.reason}<p>Storage detail: {storage.reason}</p>{/if}
  <p>Storage usage: {bytes(storage?.usage)} / {bytes(storage?.quota)}</p>
  <p>Queue: dirty {storage?.queue?.dirty ?? viewState.queued} · pending {storage?.queue?.pending ?? 0} · rejected {storage?.queue?.rejected ?? viewState.rejected}</p>
  <p>Cache: protected {bytes(storage?.cache?.protected)} · prunable {bytes(storage?.cache?.prunable)}</p>
  {#if viewState.layoutDirty}<p>Layout has a dirty local event.</p>{/if}
  {#if stores.length}
    <details>
      <summary>Local repository inventory</summary>
      <dl class="settings-inventory">
        {#each stores as [name, count]}
          <div><dt>{name}</dt><dd>{count}</dd></div>
        {/each}
      </dl>
    </details>
  {/if}
  {#if viewState.auth}<button type="button" onclick={() => actions.sync?.()}>Flush queue</button>{/if}
</section>
