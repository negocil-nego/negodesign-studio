<script lang="ts">
  import type { SelectProps } from "../data/types";

  let { id, name, options, value = $bindable(""), placeholder, disabled, required }: SelectProps = $props();
</script>

<div class="input-form">
  <select {id} {name} bind:value {disabled} {required} class="input-select">
    {#if placeholder}
      <option value="" disabled selected hidden>{placeholder}</option>
    {/if}
    {#each options as option}
      <option value={option.value} disabled={option.disabled}>{option.label}</option>
    {/each}
  </select>
</div>
