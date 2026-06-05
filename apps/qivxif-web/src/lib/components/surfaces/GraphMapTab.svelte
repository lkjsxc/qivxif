<script lang="ts">
  import { graphMapItems, visibleGraphMapEdges } from "$lib/domain/graph-map-view.ts";

  let { state: viewState, actions } = $props();
  let selectedNodeId = $state("");
  let dimensions = $state<Record<string, boolean>>({ links_to: true, references: true, profiles: true, media: true });

  const items = $derived(graphMapItems(viewState));
  const enabledKinds = $derived(Object.entries(dimensions).filter(([, on]) => on).map(([kind]) => kind));
  const edges = $derived(visibleGraphMapEdges(viewState, items, enabledKinds));
  const byNode = $derived(new Map(items.map((item) => [item.item_node_id, item])));
  const selected = $derived(items.find((item) => item.item_node_id === selectedNodeId));

  function toggle(kind: string) {
    dimensions = { ...dimensions, [kind]: !dimensions[kind] };
  }

  function selectFromKey(event: KeyboardEvent, nodeId: string) {
    if (event.key !== "Enter" && event.key !== " ") return;
    event.preventDefault();
    selectedNodeId = nodeId;
  }
</script>

<section class="tab-panel graph-map">
  <header class="surface-head">
    <div>
      <h1>Graph Map</h1>
      {#if viewState.activeGraphMapId}<p class="mono">{viewState.activeGraphMapId}</p>{/if}
    </div>
    <div class="surface-actions">
      <button type="button" onclick={() => actions.addCurrentNodeToGraphMap?.()}>Add current node</button>
      <button type="button" onclick={() => actions.linkGraphMapNodes?.()}>Create edge</button>
      <button type="button" onclick={() => actions.moveGraphMapItem?.()}>Pin first node</button>
    </div>
  </header>

  {#if viewState.activeGraphMapId}
    <div class="dimension-row" aria-label="Graph Map dimensions">
      {#each Object.keys(dimensions) as kind}
        <label>
          <input type="checkbox" checked={dimensions[kind]} onchange={() => toggle(kind)} />
          {kind}
        </label>
      {/each}
    </div>
    <div class="graph-map-layout">
      <svg class="graph-map-canvas" viewBox="0 0 720 420" role="img" aria-label="Graph Map canvas">
        {#each edges as edge}
          {@const from = byNode.get(edge.from_node)}
          {@const to = byNode.get(edge.to_node)}
          {#if from && to}
            <line x1={from.x} y1={from.y} x2={to.x} y2={to.y} class="graph-edge" />
          {/if}
        {/each}
        {#each items as item}
          <g class:selected={item.item_node_id === selectedNodeId}>
            <circle
              class="graph-node"
              role="button"
              tabindex="0"
              aria-label={`Select ${item.target_title}`}
              cx={item.x}
              cy={item.y}
              r="18"
              onclick={() => (selectedNodeId = item.item_node_id)}
              onkeydown={(event) => selectFromKey(event, item.item_node_id)}
            />
            <text x={item.x + 24} y={item.y + 4}>{item.target_title}</text>
          </g>
        {/each}
      </svg>
      <aside class="graph-map-inspector">
        {#if selected}
          <h2>{selected.target_title}</h2>
          <p class="mono">{selected.item_node_id}</p>
          <p>Kind: {selected.target_kind}</p>
          <button type="button" onclick={() => actions.openNode?.(selected.item_node_id)}>Open node</button>
        {:else}
          <h2>Inspector</h2>
          <p>Select a node circle to inspect relationships.</p>
        {/if}
        <p>Nodes: {items.length} · Edges: {edges.length}</p>
      </aside>
    </div>
  {:else}
    <p>No active Graph Map. Create one from Welcome, Editor, or New Tab.</p>
  {/if}
</section>
