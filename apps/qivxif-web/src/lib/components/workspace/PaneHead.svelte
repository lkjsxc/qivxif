<script lang="ts">
  import TabStrip from "./TabStrip.svelte";
  import TileMenu from "./TileMenu.svelte";
  import NewTabButton from "./NewTabButton.svelte";

  let { stack, state: viewState, actions, activeTab, headEl = $bindable() } = $props();

  const paneId = $derived(activeTab?.pane_node_id ?? "");
  const context = $derived({
    paneId,
    paneKind: activeTab?.pane_kind ?? "",
    targetNodeId: activeTab?.target_node_id ?? "",
  });
  const maximized = $derived(viewState.layout?.maximized_pane_id === paneId);
</script>

<header class="pane-head tile-header" bind:this={headEl}>
  <div class="pane-tabbar">
    <TabStrip {stack} {actions} />
    <NewTabButton open={() => actions.openNewTabChooser?.(paneId) ?? actions.toggleTabChooser?.(paneId)} />
  </div>
  <TileMenu {paneId} {context} {actions} {maximized} />
</header>
