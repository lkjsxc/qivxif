<script lang="ts">
  import {
    applyDropPreview,
    clearDropPreview,
    dropZoneToMove,
    measurePaneRects,
    resolvePaneDrop,
  } from "$lib/domain/drop-resolver.ts";

  const TAB_MIME = "application/x-qivxif-pane";

  let { paneId, actions, paneEl, stackEl, headEl } = $props();

  function draggedId(event: DragEvent) {
    return event.dataTransfer?.getData(TAB_MIME) || event.dataTransfer?.getData("text/plain") || "";
  }

  function stripPriority(sourceId: string, clientY: number, stripBottom: number) {
    if (!sourceId) return false;
    const strip = document.querySelector(`[data-pane-id="${sourceId}"] .tab-strip`);
    if (!strip) return false;
    const rect = strip.getBoundingClientRect();
    return clientY <= Math.max(rect.bottom, stripBottom);
  }

  function onDragOver(event: DragEvent) {
    const source = draggedId(event);
    if (!source || !paneEl || !stackEl) return;
    event.preventDefault();
    const head = headEl ?? paneEl.querySelector(".pane-head");
    if (!head) return;
    const rects = measurePaneRects(paneEl, head, stackEl);
    const result = resolvePaneDrop({
      clientX: event.clientX,
      clientY: event.clientY,
      inSourceStrip: stripPriority(source, event.clientY, rects.stripBottom),
      rects,
      targetPane: paneEl,
      targetPaneId: paneId,
    });
    applyDropPreview(paneEl, stackEl, rects, result.kind === "pane" ? result.zone : "center");
  }

  function onDragLeave(event: DragEvent) {
    if (!paneEl || !stackEl) return;
    if (!paneEl.contains(event.relatedTarget as Node)) {
      clearDropPreview(paneEl, stackEl);
    }
  }

  function onDrop(event: DragEvent) {
    const source = draggedId(event);
    if (!paneEl || !stackEl) return;
    const head = headEl ?? paneEl.querySelector(".pane-head");
    if (!head) return;
    const rects = measurePaneRects(paneEl, head, stackEl);
    const result = resolvePaneDrop({
      clientX: event.clientX,
      clientY: event.clientY,
      inSourceStrip: stripPriority(source, event.clientY, rects.stripBottom),
      rects,
      targetPane: paneEl,
      targetPaneId: paneId,
    });
    clearDropPreview(paneEl, stackEl);
    document.body.classList.remove("dragging-tab");
    if (!source) return;
    event.preventDefault();
    if (result.kind === "rail") {
      actions.movePane?.(source, result.targetPaneId, `tab-${result.insertSide}`);
      return;
    }
    actions.movePane?.(source, paneId, dropZoneToMove(result.zone));
  }
</script>

{#if paneId?.startsWith("nod_")}
  <div
    class="pane-drop-layer"
    ondragover={onDragOver}
    ondragleave={onDragLeave}
    ondrop={onDrop}
  ></div>
{/if}
