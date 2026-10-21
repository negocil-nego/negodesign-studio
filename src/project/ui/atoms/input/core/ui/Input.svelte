<script lang="ts">
  import type { InputProps } from "../data/types";

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
  {#if icon}<img src="/icons/{icon}" alt="icon" />{/if}
  {@render inputField()}
</div>
