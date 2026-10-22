<script lang="ts">
  import type { CheckboxProps } from "../data/types";

  let { id, name, checked = $bindable(false), disabled, label, required, onchange }: CheckboxProps = $props();

  function handleChange(e: Event) {
    checked = (e.target as HTMLInputElement).checked;
    onchange?.(checked);
  }
</script>

<label class="checkbox-root" for={id} class:checkbox-disabled={disabled}>
  <input
    type="checkbox"
    {id}
    {name}
    {checked}
    {disabled}
    {required}
    onchange={handleChange}
    class="checkbox-input"
  />
  <span class="checkbox-box">
    {#if checked}
      <svg viewBox="0 0 10 8" fill="none" aria-hidden="true" class="checkbox-tick">
        <path d="M1 4l3 3 5-6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    {/if}
  </span>
  {#if label}
    <span class="checkbox-label">{label}</span>
  {/if}
</label>
