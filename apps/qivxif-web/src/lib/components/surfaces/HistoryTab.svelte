<script lang="ts">
  let { state: viewState } = $props();
</script>

<section class="tab-panel history">
  <h1>History</h1>
  {#if !viewState.history?.length}
    <p>No accepted events loaded.</p>
  {:else}
    {#each viewState.history as event}
      <p>{event.kind} #{event.actor_seq}</p>
    {/each}
  {/if}
</section>
