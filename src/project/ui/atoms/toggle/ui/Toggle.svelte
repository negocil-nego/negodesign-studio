<script lang="ts">
  import type { ToggleProps } from "../data/types";
  import { StyleComponent } from "../../../../data/style.component";
  import Label from "../../label/ui/Label.svelte";

  let {
    id,
    name,
    checked = $bindable(false),
    disabled,
    label,
    labelFirst = false,
    onchange,
  }: ToggleProps = $props();

  function handleChange() {
    if (disabled) return;
    checked = !checked;
    onchange?.(checked);
  }
</script>

<label class={`${StyleComponent.toggle.root} ${disabled ? "opacity-50 cursor-not-allowed" : ""}`} for={id}>
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
    <span
      class={`${StyleComponent.toggle.thumb} ${checked ? StyleComponent.toggle.thumbOn : ""}`}
    ></span>
  </button>

  {#if label}
    <span class={StyleComponent.toggle.label}>
      {typeof label === 'string' ? label : label.label}
    </span>
  {/if}
</label>
