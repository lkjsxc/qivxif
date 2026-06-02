<script lang="ts">
  import { workspaceActions, workspaceState } from "$lib/app/workspace-context.ts";
  import AppHeader from "./AppHeader.svelte";
  import CommandPalette from "./CommandPalette.svelte";
  import TileGrid from "./TileGrid.svelte";
</script>

{#if $workspaceState}
  <section class="app-shell workspace">
    <AppHeader state={$workspaceState} actions={$workspaceActions} />
    <TileGrid state={$workspaceState} actions={$workspaceActions} />
    {#if $workspaceState.commandPaletteOpen}
      <CommandPalette state={$workspaceState} actions={$workspaceActions} />
    {/if}
  </section>
{/if}
