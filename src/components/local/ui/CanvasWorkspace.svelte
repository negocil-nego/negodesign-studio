<script lang="ts">
  import type { Snippet } from "svelte";
  let { children }: { children?: Snippet } = $props();
</script>

<div
  class="relative flex h-full w-full flex-1 items-center justify-center overflow-hidden bg-blender-base my-2"
>
  {#if children}
    {@render children()}
  {/if}
</div>

