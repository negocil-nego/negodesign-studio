<script lang="ts">
  import type { Snippet } from "svelte";
  let { children }: { children?: Snippet } = $props();
</script>

<div
  class="my-2 flex h-auto min-h-0 min-w-0 flex-1 items-start justify-center overflow-visible bg-blender-base"
>
  {#if children}
    {@render children()}
  {/if}
</div>