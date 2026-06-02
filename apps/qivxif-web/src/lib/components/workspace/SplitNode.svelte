<script lang="ts">
  import { activePaneId } from "$lib/domain/tile-tree.ts";
  import Pane from "./Pane.svelte";
  import ResizeHandle from "./ResizeHandle.svelte";
  import SplitNode from "./SplitNode.svelte";

  let { tile, state: viewState, actions } = $props();

  const axisClass = $derived(tile.axis === "column" ? "column" : "row");
  const sizes = $derived(
    tile.sizes?.length === tile.children.length
      ? tile.sizes
      : tile.children.map(() => 500),
  );
</script>

<section class="tile-split {axisClass}">
  {#each tile.children as child, index}
    <div class="tile-split-child" style:flex="{sizes[index]} 1 0">
      {#if child.kind === "split"}
        <SplitNode tile={child} {viewState} {actions} />
      {:else}
        <Pane stack={child} {viewState} {actions} />
      {/if}
    </div>
    {#if index < tile.children.length - 1}
      <ResizeHandle
        axis={tile.axis}
        onResize={(next) => {
          const paneId = activePaneId(tile.children[index]);
          actions.resizeSplit?.(paneId, next);
        }}
        {sizes}
        {index}
      />
    {/if}
  {/each}
</section>
