<script lang="ts">
  import type { InputProps } from "../data/types";
  import { getIconClass } from "../../../../../../hooks/icon.svelte";

  let { id, icon, type, required, disabled, placeholder, value }: InputProps = $props();
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

<div class="input-form">
  {#if icon}<i class={getIconClass(icon)} aria-hidden="true"></i>{/if}
  {@render inputField()}
</div>
