<script lang="ts">
  import { getIconClass, IconNamed } from "../../../../../hooks/icon.svelte";
  import { StyleComponent } from "../../../../data/style.component";
  import type { IconProps } from "../data/types";

  let { name, className, isLoading = false }: IconProps = $props();

  const iconClass = $derived(
    getIconClass(
      typeof name === "string"
        ? IconNamed[name as keyof typeof IconNamed] ?? name
        : name,
    ),
  );
</script>

{#if isLoading}
  <span class={StyleComponent.icon.spinner}></span>
{:else}
  <i class={`${iconClass} ${className} ${StyleComponent.icon.default}`} aria-hidden="true"></i>
{/if}
