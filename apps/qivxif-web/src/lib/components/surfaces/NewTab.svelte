<script lang="ts">
  import { onMount } from "svelte";

  let { state: viewState, actions } = $props();
  let panelEl: HTMLElement | undefined;

  const options = $derived([
    { id: "welcome", label: "Welcome", detail: "Open the product overview." },
    { id: "graph", label: "Graph", detail: "Inspect the active graph node." },
    { id: "editor", label: "Editor", detail: "Edit the active text node." },
    { id: "graph-map", label: "Graph Map", detail: "Explore relationships visually." },
    { id: "social", label: "Feed", detail: "Read and write local graph posts." },
    { id: "media", label: "Media", detail: "Import and inspect media assets." },
    { id: "publish", label: "Publishing", detail: "Prepare public posts." },
    { id: "sync", label: "Sync", detail: "Review queued local events." },
    { id: "settings", label: "Settings", detail: "Inspect account and storage state." },
    { id: "history", label: "History", detail: "Browse event history." },
    { id: "diagnostics", label: "Diagnostics", detail: "Inspect runtime diagnostics." },
  ]);

  onMount(() => {
    panelEl?.querySelector("button")?.focus();
  });
</script>

<section class="tab-panel new-tab-panel" bind:this={panelEl}>
  <h1>New Tab</h1>
  <p>Choose a real surface for this tab. The pane identity stays the same.</p>
  <div class="new-tab-grid">
    {#each options as option}
      <button
        type="button"
        class="new-tab-choice"
        onclick={() => actions.openTab?.(option.id)}
      >
        <strong>{option.label}</strong>
        <span>{option.detail}</span>
      </button>
    {/each}
  </div>
</section>
