<script lang="ts">
  let { open, disabled = false } = $props();
</script>

<button
  type="button"
  class="icon-button tile-add"
  aria-label="New tab"
  {disabled}
  onclick={open}
>
  +
</button>
