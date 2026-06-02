<script lang="ts">
  let { actions } = $props();

  function submit(event: Event) {
    event.preventDefault();
    const form = event.currentTarget as HTMLFormElement;
    const data = new FormData(form);
    actions.login?.(String(data.get("name") ?? ""), String(data.get("password") ?? ""));
  }
</script>

<section class="tab-panel login-panel">
  <h1>Login</h1>
  <form class="login" onsubmit={submit}>
    <label for="login-name">Login name</label>
    <input id="login-name" name="name" type="text" autocomplete="username" required />
    <label for="login-password">Password</label>
    <input id="login-password" name="password" type="password" autocomplete="current-password" required />
    <button type="submit">Login</button>
  </form>
</section>
