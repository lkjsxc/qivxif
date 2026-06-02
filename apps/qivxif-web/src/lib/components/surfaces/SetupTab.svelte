<script lang="ts">
  let { state: viewState, actions } = $props();

  function submit(event: Event) {
    event.preventDefault();
    const form = event.currentTarget as HTMLFormElement;
    const data = new FormData(form);
    actions.createOwner?.(String(data.get("name") ?? ""), String(data.get("password") ?? ""));
  }
</script>

<section class="tab-panel setup">
  <h1>Setup</h1>
  <p>Create the first owner account for this qivxif data store.</p>
  <form class="setup-form" onsubmit={submit}>
    <label for="setup-name">Name</label>
    <input id="setup-name" name="name" type="text" autocomplete="username" required />
    <label for="setup-password">Password</label>
    <input id="setup-password" name="password" type="password" autocomplete="new-password" required />
    <button type="submit" class="primary">Create owner account</button>
  </form>
  {#if viewState.setupError}
    <p class="error-text">{viewState.setupError}</p>
  {/if}
</section>
