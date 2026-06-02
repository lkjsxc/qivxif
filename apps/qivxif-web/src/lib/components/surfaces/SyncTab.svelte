<script lang="ts">
  let { state: viewState, actions } = $props();
</script>

<section class="tab-panel sync status">
  <h1>Sync Status</h1>
  <p>Network: {viewState.online ? "online" : "offline"}</p>
  <p>Dirty: {viewState.queued} · Accepted pending: {viewState.acceptedCount} · Rejected: {viewState.rejected}</p>
  {#if viewState.auth}
    <button type="button" class="primary" onclick={() => actions.sync?.()}>Flush queue</button>
  {/if}
  <div class="queue-list">
    {#each viewState.queueEntries ?? [] as entry}
      <div class="queue-entry" class:rejected={entry.status === "rejected"}>
        <span class="mono">{entry.event_id}</span>
        <span>{entry.kind} · {entry.status}</span>
        {#if entry.reason}<span class="error-text">{entry.reason}</span>{/if}
      </div>
    {/each}
  </div>
</section>
