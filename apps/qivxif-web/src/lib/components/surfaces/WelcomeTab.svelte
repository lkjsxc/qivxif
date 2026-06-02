<script lang="ts">
  let { state: viewState, actions } = $props();
</script>

<section class="tab-panel welcome">
  <h1>Welcome</h1>
  {#if viewState.auth}
    <button type="button" onclick={() => actions.createTextNode?.()}>Create text node</button>
    <button type="button" onclick={() => actions.createBoard?.()}>Create board</button>
    <button type="button" onclick={() => actions.sync?.()}>Flush queue</button>
    {#if viewState.nodes?.length}
      <div class="node-list">
        {#each viewState.nodes as node}
          <button
            type="button"
            class:selected={node.id === viewState.currentNodeId}
            onclick={() => actions.selectNode?.(node.id)}
          >
            {node.metadata_map?.title ?? node.id}{node.dirty ? " (dirty)" : ""}
          </button>
        {/each}
      </div>
    {:else}
      <p>No local nodes.</p>
    {/if}
  {:else}
    <p>Sign in to continue.</p>
    <button type="button" onclick={() => actions.openTab?.("login")}>Open login</button>
  {/if}
</section>
