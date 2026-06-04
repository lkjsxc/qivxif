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
    return value ? `${Math.round(value / 1024)} KiB` : "unknown";
  }
</script>

<section class="tab-panel settings">
  <h1>Settings</h1>
  <p>{viewState.auth ? `Signed in as ${viewState.auth.user.name}` : "Signed out"}</p>
  <p>Layout panes: {paneCount}</p>
  <p>Maximized: {viewState.layout?.maximized_pane_id ?? "none"}</p>
  <p>Storage mode: {storage?.mode ?? "unknown"}</p>
  {#if storage?.reason}<p>Storage detail: {storage.reason}</p>{/if}
  <p>Storage usage: {bytes(storage?.usage)} / {bytes(storage?.quota)}</p>
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
