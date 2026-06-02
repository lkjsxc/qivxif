<script lang="ts">
  import { visibleRoot } from "$lib/domain/tile-tree.ts";
  import SplitNode from "./SplitNode.svelte";
  import Pane from "./Pane.svelte";

  let { state: viewState, actions } = $props();

  const root = $derived(visibleRoot(viewState.layout) ?? defaultTile());

  function defaultTile() {
    return {
      kind: "stack",
      active: 0,
      tabs: [{ pane_node_id: "", pane_kind: "welcome", title: "Welcome" }],
    };
  }
</script>

<section class="tile-grid workspace-main">
  {#if root.kind === "split"}
    <SplitNode tile={root} {viewState} {actions} />
  {:else}
    <Pane stack={root} {viewState} {actions} />
  {/if}
</section>
