<script lang="ts">
  import SubmenuGridTitleMenu from "./SubmenuGridTitleMenu.svelte";
  import svelteSource from "./SubmenuGridTitleMenu.svelte?raw";
  import reactSource from "./SubmenuGridTitleMenu.tsx?raw";
  import vueSource from "./SubmenuGridTitleMenu.vue?raw";
  import flutterSource from "./SubmenuGridTitleMenu.dart?raw";
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
  {#snippet react()}<SourceCode path="src/components/local/ui/editor/terminal/menu/SubmenuGridTitleMenu.tsx" source={reactSource} />{/snippet}
  {#snippet vue()}<SourceCode path="src/components/local/ui/editor/terminal/menu/SubmenuGridTitleMenu.vue" source={vueSource} />{/snippet}
  {#snippet flutter()}<SourceCode path="src/components/local/ui/editor/terminal/menu/SubmenuGridTitleMenu.dart" source={flutterSource} />{/snippet}
</Terminal>
