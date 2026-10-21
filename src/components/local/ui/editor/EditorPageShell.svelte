<script lang="ts">
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();
</script>

<section
  class="blender-card h-full min-h-0 w-full max-w-2xl xl:max-w-3xl 2xl:max-w-5xl flex flex-col overflow-hidden transition-all duration-200"
>
  <div class="min-h-0 flex-1 overflow-y-auto p-5 my-5">
    {@render children()}
  </div>
</section>
