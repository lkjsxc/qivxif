<script lang="ts">
  import { markdownBlocks, matchCount, textStats } from "$lib/domain/markdown-preview.ts";

  let { state: viewState, actions } = $props();
  let text = $state("");
  let lastText = $state("");
  let composing = $state(false);
  let previewOpen = $state(false);
  let search = $state("");
  let history = $state<string[]>([]);
  let future = $state<string[]>([]);
  let searchEl = $state<HTMLInputElement | undefined>();

  const stats = $derived(textStats(text));
  const matches = $derived(matchCount(text, search));
  const blocks = $derived(markdownBlocks(text));
  const status = $derived(viewState.textDirty ? "local dirty" : viewState.online ? "saved" : "offline saved");

  $effect(() => {
    const incoming = viewState.text ?? "";
    if (!composing && incoming !== text && incoming !== lastText) {
      text = incoming;
      lastText = incoming;
      history = [];
      future = [];
    }
  });

  function onInput() {
    if (composing) return;
    rememberChange();
    actions.updateTextDraft?.(viewState.activePaneId, text);
  }

  function rememberChange() {
    if (lastText === text) return;
    history = [...history.slice(-49), lastText];
    future = [];
    lastText = text;
  }

  function undo() {
    const previous = history.at(-1);
    if (previous === undefined) return;
    history = history.slice(0, -1);
    future = [text, ...future.slice(0, 49)];
    text = previous;
    lastText = previous;
    actions.updateTextDraft?.(viewState.activePaneId, text);
  }

  function redo() {
    const next = future[0];
    if (next === undefined) return;
    future = future.slice(1);
    history = [...history.slice(-49), text];
    text = next;
    lastText = next;
    actions.updateTextDraft?.(viewState.activePaneId, text);
  }

  function onKeydown(event: KeyboardEvent) {
    const mod = event.metaKey || event.ctrlKey;
    if (!mod) return;
    if (event.key.toLowerCase() === "f") {
      event.preventDefault();
      searchEl?.focus();
    } else if (event.key.toLowerCase() === "z" && !event.shiftKey) {
      event.preventDefault();
      undo();
    } else if (event.key.toLowerCase() === "y" || (event.key.toLowerCase() === "z" && event.shiftKey)) {
      event.preventDefault();
      redo();
    }
  }

  function finishComposition() {
    composing = false;
    onInput();
  }
</script>

<section class="tab-panel editor-panel">
  {#if !viewState.currentNodeId}
    <h1>Text Node</h1>
    <p>Create or select a text node.</p>
  {:else}
    <div class="editor-meta">
      <span class="mono">{viewState.currentNodeId}</span>
      <span>{status}</span>
      <span>{stats.words} words · {stats.characters} chars</span>
    </div>
    <div class:preview-open={previewOpen} class="editor-workspace">
      <textarea
        class="editor"
        aria-label="Text document editor"
        bind:value={text}
        oninput={onInput}
        onkeydown={onKeydown}
        oncompositionstart={() => (composing = true)}
        oncompositionend={finishComposition}
      ></textarea>
      {#if previewOpen}
        <div class="markdown-preview" aria-label="Markdown preview">
          {#each blocks as block}
            {#if block.kind === "h1"}<h1>{block.text}</h1>
            {:else if block.kind === "h2"}<h2>{block.text}</h2>
            {:else}<p>{block.text}</p>{/if}
          {/each}
        </div>
      {/if}
    </div>
    <div class="editor-toolbar">
      <button type="button" class="primary" onclick={() => actions.saveText?.(text)}>Save text event</button>
      <button type="button" onclick={undo} disabled={history.length === 0}>Undo</button>
      <button type="button" onclick={redo} disabled={future.length === 0}>Redo</button>
      <button type="button" onclick={() => (previewOpen = !previewOpen)}>{previewOpen ? "Hide" : "Show"} preview</button>
      <button type="button" onclick={() => actions.createGraphMap?.()}>Create Graph Map</button>
      <label class="editor-search">
        Search
        <input bind:this={searchEl} bind:value={search} type="search" />
      </label>
      {#if search}<span>{matches} matches</span>{/if}
    </div>
  {/if}
</section>
