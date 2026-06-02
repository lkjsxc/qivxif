<script lang="ts">
  let { state: viewState, actions } = $props();
  let name = $state("");
  let password = $state("");

  function submit(event: Event) {
    event.preventDefault();
    actions.createOwner?.(name, password);
  }
</script>

<section class="tab-panel setup">
  <h1>Setup</h1>
  <p>Create the first owner account for this qivxif data store.</p>
  <form class="setup-form" onsubmit={submit}>
    <label>Name <input type="text" autocomplete="username" bind:value={name} /></label>
    <label>Password <input type="password" autocomplete="new-password" bind:value={password} /></label>
    <button type="submit" class="primary">Create owner account</button>
  </form>
  {#if viewState.setupError}
    <p class="error-text">{viewState.setupError}</p>
  {/if}
</section>
