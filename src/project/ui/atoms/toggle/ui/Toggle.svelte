<script lang="ts">
  import type { ToggleProps } from "../data/types";
  import { StyleComponent } from "../../../../data/style.component";

  let { id, name, checked = $bindable(false), disabled, label, onchange }: ToggleProps = $props();

  function handleChange() {
    checked = !checked;
    onchange?.(checked);
  }
</script>

<label class={StyleComponent.toggle.root} for={id}>
  <button
    type="button"
    role="switch"
    {id}
    {name}
    aria-checked={checked}
    {disabled}
    onclick={handleChange}
    class={`${StyleComponent.toggle.track} ${checked ? StyleComponent.toggle.on : ""}`}
  >
    <span class={StyleComponent.toggle.thumb}></span>
  </button>
  {#if label}
    <span class={StyleComponent.toggle.label}>{label}</span>
  {/if}
</label>
