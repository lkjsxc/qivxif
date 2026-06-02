<script lang="ts">
  import { actionsForTab, boundedActive, stateForTab } from "$lib/tab-view.ts";
  import TabSurface from "../surfaces/TabSurface.svelte";

  let { stack, state: viewState, actions } = $props();
  const activeIndex = $derived(boundedActive(stack));
</script>

<div class="tab-stack-mount">
  {#each stack.tabs as tab, index}
    {@const active = index === activeIndex}
    {@const tabState = stateForTab(viewState, tab)}
    {@const tabActions = actionsForTab(actions, tab)}
    <section
      class="tab-body"
      data-pane-id={tab.pane_node_id}
      hidden={!active}
      aria-hidden={active ? "false" : "true"}
    >
      <TabSurface state={tabState} actions={tabActions} />
    </section>
  {/each}
</div>
