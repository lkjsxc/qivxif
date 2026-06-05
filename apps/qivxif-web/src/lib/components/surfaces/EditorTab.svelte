<script lang="ts">
  let { state: viewState, actions } = $props();
  let text = $state("");

  $effect(() => {
    text = viewState.text ?? "";
  });

  function onInput() {
    actions.updateTextDraft?.(viewState.activePaneId, text);
  }
</script>

<section class="tab-panel editor-panel">
  {#if !viewState.currentNodeId}
    <h1>Text Node</h1>
    <p>Create or select a text node.</p>
  {:else}
    <div class="editor-meta">
      <span class="mono">{viewState.currentNodeId}</span>
      {#if viewState.textDirty}<span>Dirty</span>{/if}
    </div>
    <textarea class="editor" bind:value={text} oninput={onInput}></textarea>
    <div>
      <button type="button" class="primary" onclick={() => actions.saveText?.(text)}>Save text event</button>
      <button type="button" onclick={() => actions.createGraphMap?.()}>Create Graph Map</button>
    </div>
  {/if}
</section>
