<script lang="ts">
  import type { Snippet } from "svelte";
  let { children }: { children?: Snippet } = $props();
</script>

<div
  class="flex-1 h-full w-full overflow-hidden relative bg-blender-base"
  style="background-image: radial-gradient(circle, #2d3036 1px, transparent 1px); background-size: 24px 24px;"
>
  {#if children}
    {@render children()}
  {/if}
</div>
