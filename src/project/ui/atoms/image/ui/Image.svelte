<script lang="ts">
  import Skeleton from "../../skeleton/ui/Skeleton.svelte";
  import { StyleComponent } from "../../../../data/style.component";
  import type { ImageProps } from "../data/types";

  let { src, isLoading, onClick, className }: ImageProps = $props();
</script>

{#if isLoading}
  <Skeleton className={className || StyleComponent.image.default} />
{:else if src}
  <button onclick={onClick} class={className}>
    <img {src} alt={src} class={className || StyleComponent.image.default} />
  </button>
{/if}
