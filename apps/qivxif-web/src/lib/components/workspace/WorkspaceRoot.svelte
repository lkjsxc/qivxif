<script lang="ts">
  import { workspaceActions, workspaceState } from "$lib/app/workspace-context.ts";
  import AppHeader from "./AppHeader.svelte";
  import CommandPalette from "./CommandPalette.svelte";
  import TileGrid from "./TileGrid.svelte";

  const viewState = $derived($workspaceState);
  const actions = $derived($workspaceActions);
</script>

{#if viewState}
  <section class="app-shell workspace">
    <AppHeader {viewState} {actions} />
    <TileGrid {viewState} {actions} />
    {#if viewState.commandPaletteOpen}
      <CommandPalette {viewState} {actions} />
    {/if}
  </section>
{/if}
