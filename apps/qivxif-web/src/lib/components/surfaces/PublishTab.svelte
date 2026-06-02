<script lang="ts">
  let { state: viewState, actions } = $props();
  const hasDraft = $derived(Boolean(viewState.currentBlogPostId));
</script>

<section class="tab-panel publish">
  <h1>Publishing</h1>
  <form
    class="publish-draft-form"
    onsubmit={(event) => {
      event.preventDefault();
      const data = new FormData(event.currentTarget as HTMLFormElement);
      actions.createBlogDraft?.(String(data.get("title") ?? ""));
    }}
  >
    <label for="publish-title">Blog title</label>
    <input id="publish-title" name="title" type="text" required />
    <button type="submit">Create blog draft</button>
  </form>
  <p>Draft: {viewState.currentBlogPost?.metadata_map?.title ?? viewState.currentBlogPostId ?? "none"}</p>
  <form
    class="publish-submit-form"
    onsubmit={(event) => {
      event.preventDefault();
      if (!hasDraft) return;
      const data = new FormData(event.currentTarget as HTMLFormElement);
      actions.publishBlogPost?.(String(data.get("slug") ?? ""), String(data.get("summary") ?? ""));
    }}
  >
    <label for="publish-slug">Slug</label>
    <input id="publish-slug" name="slug" type="text" required disabled={!hasDraft} />
    <label for="publish-summary">Summary</label>
    <input id="publish-summary" name="summary" type="text" required disabled={!hasDraft} />
    <button type="submit" disabled={!hasDraft}>Publish draft</button>
  </form>
  {#if hasDraft}
    <button type="button" onclick={() => actions.unpublishBlogPost?.()}>Unpublish</button>
  {/if}
  {#if viewState.lastPublicRoute}
    <p class="mono">{viewState.lastPublicRoute}</p>
  {/if}
</section>
