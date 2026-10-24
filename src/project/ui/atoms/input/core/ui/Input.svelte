<script lang="ts">
  import type { InputProps } from "../data/types";
  import { getIconClass } from "../../../../../../hooks/icon.svelte";
  import { StyleComponent } from "../../../../../data/style.component";

  let { id, icon, type, required, disabled, placeholder, value }: InputProps = $props();

  const fieldClass = $derived(
    (StyleComponent.input as unknown as Record<string, { root?: string }>)[type ?? ""]
      ?.root ?? StyleComponent.input.form,
  );
</script>

{#snippet inputField()}
  <input
    {id}
    {type}
    {required}
    {disabled}
    {placeholder}
    {value}
    data-name="label"
  />
{/snippet}

<div class={fieldClass}>
  {#if icon}<i class={getIconClass(icon)} aria-hidden="true"></i>{/if}
  {@render inputField()}
</div>
