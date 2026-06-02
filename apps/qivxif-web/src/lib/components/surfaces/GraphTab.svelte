<script lang="ts">
  let { state: viewState, actions } = $props();

  function openNode(event: Event) {
    event.preventDefault();
    const form = event.currentTarget as HTMLFormElement;
    const data = new FormData(form);
    actions.openNode?.(String(data.get("nodeId") ?? "").trim());
  }
</script>

<section class="tab-panel graph">
  <h1>Graph</h1>
  <form class="open-node" onsubmit={openNode}>
    <label for="graph-node-id">Server node id</label>
    <input id="graph-node-id" name="nodeId" type="text" required />
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
