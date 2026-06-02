<script lang="ts">
  import { tabInsertSide } from "$lib/domain/drop-resolver.ts";

  const TAB_MIME = "application/x-qivxif-pane";

  let { tab, label, active, onFocus, onMove } = $props();

  let dropSide = $state<string | undefined>();

  function onDragStart(event: DragEvent) {
    if (!tab.pane_node_id?.startsWith("nod_")) return;
    event.dataTransfer?.setData(TAB_MIME, tab.pane_node_id);
    event.dataTransfer?.setData("text/plain", tab.pane_node_id);
    event.dataTransfer!.effectAllowed = "move";
    document.body.classList.add("dragging-tab");
  }

  function onDragEnd() {
    document.body.classList.remove("dragging-tab");
    dropSide = undefined;
  }

  function onDragOver(event: DragEvent) {
    const source = event.dataTransfer?.getData(TAB_MIME) || event.dataTransfer?.getData("text/plain");
    if (!source) return;
    event.preventDefault();
    event.stopPropagation();
    dropSide = tabInsertSide(event.currentTarget as HTMLElement, event.clientX);
  }

  function onDrop(event: DragEvent) {
    const source = event.dataTransfer?.getData(TAB_MIME) || event.dataTransfer?.getData("text/plain");
    const side = tabInsertSide(event.currentTarget as HTMLElement, event.clientX);
    dropSide = undefined;
    if (!source) return;
    event.preventDefault();
    event.stopPropagation();
    onMove(source, `tab-${side}`);
  }
</script>

<button
  type="button"
  class="tab tab-frame"
  class:active
  role="tab"
  aria-selected={active}
  data-pane-id={tab.pane_node_id}
  data-tab-kind={tab.pane_kind}
  data-drop-side={dropSide}
  draggable={tab.pane_node_id?.startsWith("nod_") ? true : undefined}
  onclick={onFocus}
  ondragstart={onDragStart}
  ondragend={onDragEnd}
  ondragover={onDragOver}
  ondrop={onDrop}
>
  {label}
</button>
