<script lang="ts">
  import type { Snippet } from "svelte";
  import { bootswatchUi } from "../data/bootswatch-ui.svelte";

  let { children }: { children?: Snippet } = $props();
</script>

<div
  data-bootswatch={bootswatchUi.projectDataAttr}
  class="flex min-h-0 min-w-0 flex-1 items-start justify-center overflow-x-hidden overflow-y-auto bg-blender-base"
>
  {#if children}
    {@render children()}
  {/if}
</div>
