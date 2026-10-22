<script lang="ts">
  import type { ToggleProps } from "../data/types";

  let { id, name, checked = $bindable(false), disabled, label, onchange }: ToggleProps = $props();

  function handleChange() {
    checked = !checked;
    onchange?.(checked);
  }
</script>

<label class="toggle-root" for={id}>
  <button
    type="button"
    role="switch"
    {id}
    {name}
    aria-checked={checked}
    {disabled}
    onclick={handleChange}
    class="toggle-track"
    class:toggle-on={checked}
  >
    <span class="toggle-thumb"></span>
  </button>
  {#if label}
    <span class="toggle-label">{label}</span>
  {/if}
</label>
