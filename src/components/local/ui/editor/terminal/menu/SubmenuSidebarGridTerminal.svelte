<script lang="ts">
  import SubmenuSidebarGridMenu from "./SubmenuSidebarGridMenu.svelte";
  import svelteSource from "./SubmenuSidebarGridMenu.svelte?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="menu-sidebar-grid">
  {#snippet preview()}
    <div class="min-h-[68vh] overflow-hidden rounded-xl border border-blender-border bg-blender-base">
      <SubmenuSidebarGridMenu />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/components/local/ui/editor/terminal/menu/SubmenuSidebarGridMenu.svelte"
      source={svelteSource}
    />
  {/snippet}
</Terminal>
