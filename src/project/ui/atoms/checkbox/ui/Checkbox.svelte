<script lang="ts">
  import type { CheckboxProps } from "../data/types";
  import { StyleComponent } from "../../../../data/style.component";

  let { id, name, checked = $bindable(false), disabled, label, required, onchange }: CheckboxProps = $props();

  function handleChange(e: Event) {
    checked = (e.target as HTMLInputElement).checked;
    onchange?.(checked);
  }
</script>

<label class={`${StyleComponent.checkbox.root} ${disabled ? StyleComponent.checkbox.disabled : ""}`} for={id}>
  <input
    type="checkbox"
    {id}
    {name}
    {checked}
    {disabled}
    {required}
    onchange={handleChange}
    class={StyleComponent.checkbox.input}
  />
  <span class={StyleComponent.checkbox.box}>
    {#if checked}
      <svg viewBox="0 0 10 8" fill="none" aria-hidden="true" class={StyleComponent.checkbox.tick}>
        <path d="M1 4l3 3 5-6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    {/if}
  </span>
  {#if label}
    <span class={StyleComponent.checkbox.label}>{label}</span>
  {/if}
</label>
