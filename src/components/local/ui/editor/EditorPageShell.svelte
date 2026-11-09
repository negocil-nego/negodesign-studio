<script lang="ts">
  import type { Snippet } from "svelte";
  import { LocalStyleComponent } from "../../data/style.component";

  let { children }: { children: Snippet } = $props();
</script>

<section
  class="{LocalStyleComponent.layout
    .card} min-h-0 w-full max-w-2xl overflow-visible xl:max-w-3xl 2xl:max-w-7xl flex flex-col transition-all duration-200"
>
  <div class="min-h-0 flex-1 p-5 my-5">
    {@render children()}
  </div>
</section>
