<script lang="ts">
  let { state: viewState, actions } = $props();
  let body = $state("");

  const targets = $derived(
    (viewState.nodes ?? []).filter(
      (node) => node.kind === "profile" && node.id !== viewState.auth?.user?.profile_node_id,
    ),
  );

  const edges = $derived(
    (viewState.edges ?? []).filter(
      (edge) =>
        edge.from_node === viewState.auth?.user?.profile_node_id &&
        ["follows", "mutes", "blocks"].includes(edge.kind) &&
        !edge.tombstone,
    ),
  );

  function submitPost(event: Event) {
    event.preventDefault();
    if (!body.trim()) return;
    actions.createShortPost?.(body);
    body = "";
  }

  function initials(name: string) {
    return (name?.trim()?.[0] ?? "?").toUpperCase();
  }

  function clearKind(kind: string) {
    const map: Record<string, string> = {
      blocks: "social.unblock",
      follows: "social.unfollow",
      mutes: "social.unmute",
    };
    return map[kind];
  }
</script>

<section class="social tab-panel">
  <h1>Feed</h1>
  <p class="social-context">
    Current profile: {viewState.auth?.user?.profile_node_id ?? "signed out"}
  </p>

  {#if viewState.auth}
    <form class="feed-compose" onsubmit={submitPost}>
      <textarea rows="3" placeholder="Write a short post" bind:value={body}></textarea>
      <button type="submit" class="primary" disabled={!body.trim()}>Post</button>
    </form>
  {/if}

  {#if !viewState.feedItems?.length}
    <p>Home feed is empty. Follow profiles to see posts here.</p>
  {:else}
    {#each viewState.feedItems as entry}
      {@const item = entry.item ?? entry}
      <article class="feed-card feed-item">
        <div class="feed-avatar">{initials(item.author_name)}</div>
        <div class="feed-card-head">
          <span class="feed-author">{item.author_name ?? "unknown"}</span>
          <span class="feed-meta">{item.created_at ?? ""}</span>
        </div>
        <p class="feed-body">{item.body ?? ""}</p>
      </article>
    {/each}
  {/if}

  <h2>Profile targets</h2>
  {#if !viewState.auth?.user?.profile_node_id}
    <p>Sign in with a profile to manage relationships.</p>
  {:else if !targets.length}
    <p>No discovered profile targets.</p>
  {:else}
    {#each targets as target}
      <div class="relationship-row">
        <span>{target.metadata_map?.display_name ?? target.id}</span>
        <button type="button" onclick={() => actions.followProfile?.(target.id)}>Follow</button>
        <button type="button" onclick={() => actions.muteProfile?.(target.id)}>Mute</button>
        <button type="button" onclick={() => actions.blockProfile?.(target.id)}>Block</button>
      </div>
    {/each}
  {/if}

  <h2>Relationship edges</h2>
  {#if !edges.length}
    <p>No local relationship edges.</p>
  {:else}
    {#each edges as edge}
      <div class="relationship-row">
        <span>{edge.kind}: {edge.to_node}</span>
        <button type="button" onclick={() => actions.clearSocialEdge?.(edge.id, clearKind(edge.kind))}>
          Clear
        </button>
      </div>
    {/each}
  {/if}
</section>
