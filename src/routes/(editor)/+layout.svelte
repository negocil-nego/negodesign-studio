<script lang="ts">
  import MenuNavigation from "../../components/local/ui/MenuNavigation.svelte";

  let { children } = $props();
</script>

<div
  class="flex h-dvh min-h-0 flex-col overflow-hidden bg-blender-base font-commissioner text-slate-900 dark:text-zinc-100"
>
  <div class="z-40 flex-none bg-blender-base">
    <MenuNavigation />
  </div>
  {@render children()}
</div>

<style>
  :global(html),
  :global(body) {
    height: 100%;
    overflow: hidden;
  }
</style>
