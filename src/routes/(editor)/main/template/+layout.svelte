<script lang="ts">
  import { onMount } from "svelte";
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();

  onMount(() => {
    document.documentElement.dataset.theme =
      localStorage.getItem("negodesign-theme") === "dark" ? "dark" : "light";
  });
</script>

<main class="min-h-0 w-full flex-1 overflow-y-auto">
  {@render children()}
</main>
