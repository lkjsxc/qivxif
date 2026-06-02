<script lang="ts">
  let { state: viewState, actions } = $props();
  let nodeId = $state("");

  function openNode(event: Event) {
    event.preventDefault();
    actions.openNode?.(nodeId.trim());
  }
</script>

<section class="tab-panel graph">
  <h1>Graph</h1>
  <form class="open-node" onsubmit={openNode}>
    <label>Server node id <input type="text" bind:value={nodeId} /></label>
    <button type="submit">Open node</button>
  </form>
  <div class="node-list">
    {#if !viewState.nodes?.length}
      <p>No local nodes.</p>
    {:else}
      {#each viewState.nodes as node}
        <button
          type="button"
          class:selected={node.id === viewState.currentNodeId}
          onclick={() => actions.selectNode?.(node.id)}
        >
          {node.metadata_map?.title ?? node.id}
        </button>
      {/each}
    {/if}
  </div>
</section>
