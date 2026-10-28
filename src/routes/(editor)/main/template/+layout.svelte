<script lang="ts">
  import { onMount } from "svelte";
  import type { Snippet } from "svelte";
  import MenuNavigation from "../../../../components/local/ui/MenuNavigation.svelte";

  let { children }: { children: Snippet } = $props();

  onMount(() => {
    document.documentElement.dataset.theme =
      localStorage.getItem("negodesign-theme") === "dark" ? "dark" : "light";
  });
</script>

<div class="flex h-dvh min-h-0 flex-col overflow-hidden bg-blender-base font-commissioner text-slate-900 dark:text-zinc-100">
  <div class="z-40 flex-none bg-blender-base">
    <MenuNavigation />
  </div>

  <main class="min-h-0 w-full flex-1 overflow-y-auto">
    {@render children()}
  </main>
</div>
