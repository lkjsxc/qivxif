<script lang="ts">
  import { boundedActive, tabLabel } from "$lib/tab-view.ts";
  import TabFrame from "./TabFrame.svelte";

  let { stack, actions } = $props();

  let railEl: HTMLDivElement | undefined;
  const activeIndex = $derived(boundedActive(stack));
</script>

<div class="tab-rail tab-strip" role="tablist" bind:this={railEl}>
  {#each stack.tabs as tab, index}
    <TabFrame
      {tab}
      active={index === activeIndex}
      label={tabLabel(tab)}
      onFocus={() => actions.focusPane?.(tab.pane_node_id)}
      onMove={(source, zone) => actions.movePane?.(source, tab.pane_node_id, zone)}
    />
  {/each}
</div>
