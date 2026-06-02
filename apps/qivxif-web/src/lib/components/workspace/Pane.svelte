<script lang="ts">
  import { boundedActive } from "$lib/tab-view.ts";
  import PaneHead from "./PaneHead.svelte";
  import PaneDropLayer from "./PaneDropLayer.svelte";
  import PaneTabStack from "./PaneTabStack.svelte";

  let { stack, state: viewState, actions } = $props();

  let paneEl: HTMLElement | undefined;
  let headEl: HTMLElement | undefined;
  let stackEl: HTMLElement | undefined;

  const activeIndex = $derived(boundedActive(stack));
  const activeTab = $derived(stack.tabs[activeIndex] ?? null);
  const paneId = $derived(activeTab?.pane_node_id ?? "");
</script>

<article class="pane tile" data-pane-id={paneId || undefined} bind:this={paneEl}>
  <PaneHead bind:headEl {stack} state={viewState} {actions} {activeTab} />
  <section class="pane-stack" bind:this={stackEl}>
    <PaneDropLayer {paneId} {actions} {paneEl} {stackEl} {headEl} />
    <PaneTabStack {stack} state={viewState} {actions} />
  </section>
</article>
