<script lang="ts">
  import { boardItems } from "$lib/effects/board-actions.ts";

  let { state: viewState, actions } = $props();
  const items = $derived(boardItems(state));
</script>

<section class="tab-panel board">
  <h1>Board</h1>
  {#if viewState.activeBoardId}
    <p class="mono">{viewState.activeBoardId}</p>
    <p>Items: {items.length}</p>
    <button type="button" onclick={() => actions.addCurrentNodeToBoard?.()}>Add current node</button>
    <button type="button" onclick={() => actions.linkBoardNodes?.()}>Link board nodes</button>
    <button type="button" onclick={() => actions.moveBoardItem?.()}>Move board item</button>
  {:else}
    <p>No active board. Create one from Welcome or Editor.</p>
  {/if}
</section>
