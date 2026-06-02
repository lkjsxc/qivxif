<script lang="ts">
  let { state: viewState, actions } = $props();
  let title = $state("");
  let slug = $state("");
  let summary = $state("");
</script>

<section class="tab-panel publish">
  <h1>Publishing</h1>
  <form
    onsubmit={(event) => {
      event.preventDefault();
      actions.createBlogDraft?.(title);
    }}
  >
    <label>Blog title <input type="text" bind:value={title} /></label>
    <button type="submit">Create blog draft</button>
  </form>
  {#if viewState.currentBlogPostId}
    <p>Draft: {viewState.currentBlogPost?.metadata_map?.title ?? viewState.currentBlogPostId}</p>
    <form
      onsubmit={(event) => {
        event.preventDefault();
        actions.publishBlogPost?.(slug, summary);
      }}
    >
      <label>Slug <input type="text" bind:value={slug} /></label>
      <label>Summary <input type="text" bind:value={summary} /></label>
      <button type="submit">Publish draft</button>
    </form>
    <button type="button" onclick={() => actions.unpublishBlogPost?.()}>Unpublish</button>
  {/if}
  {#if viewState.lastPublicRoute}
    <p class="mono">{viewState.lastPublicRoute}</p>
  {/if}
</section>
