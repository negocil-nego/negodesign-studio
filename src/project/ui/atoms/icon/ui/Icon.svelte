<script lang="ts">
  import { getIconClass, IconNamed } from "../../../../../hooks/icon.svelte";
  import type { IconProps } from "../data/types";

  let { name, className, isLoading = false }: IconProps = $props();

  const iconClass = $derived(getIconClass(IconNamed[name]));
</script>

{#if isLoading}
  <span class="animate-spin inline-block w-5 h-5"></span>
{:else}
  <i class={`${iconClass} ${className} text-base opacity-75`} aria-hidden="true"></i>
{/if}
