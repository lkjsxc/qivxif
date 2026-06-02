<script lang="ts">
  import { onMount } from "svelte";
  import { createController } from "$lib/app/controller.ts";
  import { bindController } from "$lib/app/workspace-context.ts";
  import WorkspaceRoot from "$lib/components/workspace/WorkspaceRoot.svelte";

  let ready = $state(false);

  onMount(async () => {
    const controller = await createController();
    bindController(controller);
    await controller.start();
    ready = true;
  });
</script>

{#if ready}
  <WorkspaceRoot />
{:else}
  <main class="app-shell workspace" aria-busy="true">
    <p class="brand">qivxif</p>
  </main>
{/if}
