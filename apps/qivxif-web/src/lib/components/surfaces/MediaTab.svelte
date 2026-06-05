<script lang="ts">
  import { formatBytes, mediaAssets, mediaAttachments } from "$lib/domain/media-view.ts";

  let { state: viewState, actions } = $props();
  let selectedId = $state("");
  const assets = $derived(mediaAssets(viewState));
  const selected = $derived(assets.find((asset: any) => asset.id === selectedId) ?? assets[0]);
  const attachments = $derived(selected ? mediaAttachments(viewState, selected.id) : []);

  async function importFile(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;
    await actions.importMediaFile?.(file);
    input.value = "";
  }
</script>

<section class="tab-panel media-panel">
  <header class="surface-head">
    <div>
      <h1>Media</h1>
      <p>Chunked local imports, graph attachments, and cache diagnostics.</p>
    </div>
    <label class="media-import">
      Import file
      <input type="file" onchange={importFile} />
    </label>
  </header>

  {#if assets.length}
    <div class="media-layout">
      <div class="media-list">
        {#each assets as asset}
          <button
            type="button"
            class:selected={asset.id === selected?.id}
            onclick={() => (selectedId = asset.id)}
          >
            <strong>{asset.filename ?? asset.title ?? asset.id}</strong>
            <span>{formatBytes(asset.size)}</span>
          </button>
        {/each}
      </div>
      <div class="media-detail">
        <h2>{selected.filename ?? selected.title ?? selected.id}</h2>
        <p class="mono">{selected.id}</p>
        <p>Type: {selected.mime_type ?? "unknown"}</p>
        <p>Size: {formatBytes(selected.size)}</p>
        <p>Hash: <span class="mono">{selected.content_hash ?? "pending"}</span></p>
        <p>Chunks: {selected.chunks?.length ?? "unknown"}</p>
        <p>Attachments: {attachments.length}</p>
        <button type="button" onclick={() => actions.attachMediaToNode?.(selected.id)}>Attach to current node</button>
      </div>
    </div>
  {:else}
    <p>No media assets. Import a file to create a real media asset node.</p>
  {/if}
</section>
