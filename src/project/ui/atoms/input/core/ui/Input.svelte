<script lang="ts">
  import type { InputProps } from "../data/types";

  let { icon, type, required, disabled, placeholder, value }: InputProps =
    $props();
</script>

{#snippet inputField()}
  <input
    {type}
    {required}
    {disabled}
    {placeholder}
    {value}
    data-name="label"
  />
{/snippet}

{#if icon}
  <div class="flex items-center">
    <img src="/icons/{icon}" alt="icon" />
    {@render inputField()}
  </div>
{:else}
  {@render inputField()}
{/if}
