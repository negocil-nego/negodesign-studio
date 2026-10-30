<script lang="ts">
  import SubmenuGridTitleMenu from "./SubmenuGridTitleMenu.svelte";
  import svelteSource from "./SubmenuGridTitleMenu.svelte?raw";
  import Terminal from "../Terminal.svelte";
  import SourceCode from "../SourceCode.svelte";
</script>

<Terminal id="submenu-grid-title">
  {#snippet preview()}
    <div class="min-h-[68vh] overflow-hidden rounded-xl border border-blender-border bg-blender-base">
      <SubmenuGridTitleMenu />
    </div>
  {/snippet}

  {#snippet svelte()}
    <SourceCode
      path="src/components/local/ui/editor/terminal/menu/SubmenuGridTitleMenu.svelte"
      source={svelteSource}
    />
  {/snippet}
</Terminal>
