<script lang="ts">
  import { onMount } from "svelte";
  import { createController } from "$lib/app/controller.ts";
  import { bindController } from "$lib/app/workspace-context.ts";
  import WorkspaceRoot from "$lib/components/workspace/WorkspaceRoot.svelte";

  onMount(async () => {
    try {
      const controller = await createController();
      bindController(controller);
      await controller.start();
    } catch (error) {
      console.error(error);
    }
  });
</script>

<WorkspaceRoot />
