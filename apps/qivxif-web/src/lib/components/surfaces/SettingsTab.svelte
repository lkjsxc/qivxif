<script lang="ts">
  let { state: viewState, actions } = $props();

  const paneCount = $derived(countPanes(viewState.layout?.root));

  function countPanes(tile) {
    if (!tile) return 0;
    if (tile.kind === "stack") return tile.tabs.length;
    return (tile.children ?? []).reduce((sum, child) => sum + countPanes(child), 0);
  }
</script>

<section class="tab-panel settings">
  <h1>Settings</h1>
  <p>{viewState.auth ? `Signed in as ${viewState.auth.user.name}` : "Signed out"}</p>
  <p>Layout panes: {paneCount}</p>
  <p>Maximized: {viewState.layout?.maximized_pane_id ?? "none"}</p>
  {#if viewState.layoutDirty}<p>Layout has a dirty local event.</p>{/if}
</section>
