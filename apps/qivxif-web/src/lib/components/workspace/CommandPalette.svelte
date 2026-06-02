<script lang="ts">
  import { boardItems } from "$lib/effects/board-actions.ts";
  import { visibleRoot } from "$lib/domain/tile-tree.ts";

  let { state: viewState, actions } = $props();
  let query = $state("");

  const paneId = $derived(activeTab()?.pane_node_id ?? "");
  const context = $derived(paneContext(activeTab()));
  const commands = $derived(buildCommands());
  const filtered = $derived(
    commands.filter((cmd) => {
      const needle = query.trim().toLowerCase();
      if (!needle) return true;
      return `${cmd.label} ${cmd.reason}`.toLowerCase().includes(needle);
    }),
  );

  function buildCommands() {
    const authed = Boolean(viewState.auth);
    const items = boardItems(viewState);
    const hasBoardTarget = Boolean(viewState.activeBoardId && viewState.currentNodeId);
    return [
      cmd("Open graph", true, () => actions.openTab?.("graph", paneId)),
      cmd("Create text node", authed, () => actions.createTextNode?.(), "login required"),
      cmd("Create board", authed, () => actions.createBoard?.(context), "login required"),
      cmd("Split pane", authed && paneId, () => actions.splitPane?.(paneId, context), "pane required"),
      cmd("Open sync status", true, () => actions.openTab?.("sync", paneId)),
      cmd("Open feed", true, () => actions.openTab?.("social", paneId)),
      cmd("Flush queue", authed, () => actions.sync?.(), "login required"),
      cmd(
        "Add current node to board",
        authed && hasBoardTarget,
        () => actions.addCurrentNodeToBoard?.(),
        "board required",
      ),
      cmd("Move board item", authed && items.length > 0, () => actions.moveBoardItem?.(), "item required"),
    ];
  }

  function cmd(label, enabled, run, reason = "") {
    return { label, enabled: Boolean(enabled), run, reason: enabled ? "" : reason };
  }

  function activeTab() {
    const root = visibleRoot(viewState.layout);
    return tabByPane(root, viewState.activePaneId) ?? firstStack(root)?.tabs?.[0] ?? null;
  }

  function tabByPane(tile, id) {
    if (!tile || !id) return null;
    if (tile.kind === "stack") return tile.tabs.find((tab) => tab.pane_node_id === id) ?? null;
    for (const child of tile.children ?? []) {
      const found = tabByPane(child, id);
      if (found) return found;
    }
    return null;
  }

  function firstStack(tile) {
    if (!tile) return null;
    if (tile.kind === "stack") return tile;
    for (const child of tile.children ?? []) {
      const found = firstStack(child);
      if (found) return found;
    }
    return null;
  }

  function paneContext(tab) {
    return {
      paneId: tab?.pane_node_id ?? "",
      paneKind: tab?.pane_kind ?? "",
      targetNodeId: tab?.target_node_id ?? "",
    };
  }

  async function run(cmdItem) {
    if (!cmdItem.enabled) return;
    await cmdItem.run();
    actions.toggleCommandPalette?.(false);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div
  class="command-palette-backdrop"
  onclick={(event) => {
    if (event.target === event.currentTarget) actions.toggleCommandPalette?.(false);
  }}
>
  <div class="command-palette" role="dialog" aria-label="Command palette">
    <h2>Command palette</h2>
    <input class="command-search" type="search" aria-label="Search commands" bind:value={query} />
    <div class="command-list">
      {#each filtered as command}
        <button
          type="button"
          class="command"
          disabled={!command.enabled}
          title={command.reason}
          onclick={() => run(command)}
        >
          {command.label}
          {#if command.reason}<span class="command-reason">{command.reason}</span>{/if}
        </button>
      {/each}
    </div>
    {#if filtered.length === 0}
      <p class="command-empty">No matching commands.</p>
    {/if}
  </div>
</div>
