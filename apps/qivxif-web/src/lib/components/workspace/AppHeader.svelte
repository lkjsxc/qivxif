<script lang="ts">
  let { state: viewState, actions } = $props();

  const caps = $derived(
    !viewState.capabilities?.length ? "none" : viewState.capabilities.join(", "),
  );
  const authLabel = $derived(
    viewState.setupRequired
      ? "Setup required"
      : viewState.auth
        ? `Signed in as ${viewState.auth.user.name}`
        : "Signed out",
  );
</script>

<header class="app-header">
  <div class="brand">qivxif</div>
  <div class="header-status">
    <span>Sync: {viewState.online ? "online" : "offline"}</span>
    <span>{authLabel}</span>
    <span>Queued: {viewState.queued}</span>
    <span>Rejected: {viewState.rejected ?? 0}</span>
    <span>Capabilities: {caps}</span>
    {#if viewState.lastError}<span>Error: {viewState.lastError}</span>{/if}
  </div>
  <div class="header-actions">
    <button type="button" class="header-button" onclick={() => actions.toggleCommandPalette?.()}>
      Commands
    </button>
    <button type="button" class="header-button" onclick={() => actions.toggleTabChooser?.()}>
      New tab
    </button>
    {#if viewState.auth}
      <button type="button" class="header-button" onclick={() => actions.sync?.()}>Sync</button>
    {/if}
  </div>
</header>
