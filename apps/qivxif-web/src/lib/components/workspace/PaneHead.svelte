<script lang="ts">
  import TabStrip from "./TabStrip.svelte";

  let { stack, state: viewState, actions, activeTab, headEl = $bindable() } = $props();

  const chooserTabs = [
    { id: "welcome", label: "Welcome" },
    { id: "graph", label: "Graph" },
    { id: "editor", label: "Editor" },
    { id: "board", label: "Board" },
    { id: "social", label: "Feed" },
    { id: "publish", label: "Publishing" },
    { id: "sync", label: "Sync" },
    { id: "settings", label: "Settings" },
    { id: "history", label: "History" },
    { id: "diagnostics", label: "Diagnostics" },
  ];

  const paneId = $derived(activeTab?.pane_node_id ?? "");
  const context = $derived({
    paneId,
    paneKind: activeTab?.pane_kind ?? "",
    targetNodeId: activeTab?.target_node_id ?? "",
  });
  const chooserOpen = $derived(viewState.tabChooserOpen && viewState.tabChooserPaneId === paneId);
</script>

<header class="pane-head tile-header" bind:this={headEl}>
  <TabStrip {stack} {actions} />
  <div class="tile-controls">
    <button type="button" class="icon-button tile-add" onclick={() => actions.toggleTabChooser?.(paneId)}>+</button>
    <button type="button" class="tile-menu" onclick={() => actions.splitPane?.(paneId, context)}>Split pane</button>
    <button type="button" class="tile-menu" onclick={() => actions.stackTab?.(paneId, context)}>Stack tab</button>
    <button type="button" class="tile-menu" onclick={() => actions.maximizePane?.(paneId)}>Maximize pane</button>
    <button type="button" class="tile-menu" onclick={() => actions.closePane?.(paneId)}>Close pane</button>
  </div>
  {#if chooserOpen}
    <div class="tab-chooser" role="menu">
      {#each chooserTabs as tab}
        <button type="button" class="chooser-tab" onclick={() => actions.openTab?.(tab.id, paneId)}>{tab.label}</button>
      {/each}
    </div>
  {/if}
</header>
