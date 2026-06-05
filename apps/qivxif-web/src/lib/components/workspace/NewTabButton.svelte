<script lang="ts">
  let { open, disabled = false } = $props();
</script>

<button
  type="button"
  class="icon-button tile-add"
  aria-label="Add tab"
  {disabled}
  onclick={open}
>
  +
</button>
