<script lang="ts">
  let { actions } = $props();
  let name = $state("");
  let password = $state("");

  function submit(event: Event) {
    event.preventDefault();
    actions.login?.(name, password);
  }
</script>

<section class="tab-panel login-panel">
  <h1>Login</h1>
  <form class="login" onsubmit={submit}>
    <label>Login name <input type="text" autocomplete="username" bind:value={name} /></label>
    <label>Password <input type="password" autocomplete="current-password" bind:value={password} /></label>
    <button type="submit">Login</button>
  </form>
</section>
