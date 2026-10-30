<script lang="ts">
  import type { Snippet } from "svelte";
  import ComponentsSidebar from "../../../../components/local/ui/ComponentsSidebar.svelte";
  import InspectorSidebar from "../../../../components/local/ui/InspectorSidebar.svelte";
  import CanvasWorkspace from "../../../../components/local/ui/CanvasWorkspace.svelte";
  import MenuNavigation from "../../../../components/local/ui/MenuNavigation.svelte";
  import { StyleComponent } from "../../../../components/local/data/style.component";

  let { children }: { children: Snippet } = $props();
</script>

<div
  class="{StyleComponent.layout
    .app} h-auto! min-h-dvh! overflow-visible! flex flex-col"
>
  <div class="sticky top-0 z-40 flex-none bg-blender-base">
    <MenuNavigation />
  </div>
  <!-- Main editor area -->
  <div class="flex flex-1 items-start overflow-visible">
    <!-- Left: Components Library sidebar -->
    <div
      class="sticky top-7 z-20 h-[calc(100dvh-1.75rem)] flex-none overflow-hidden self-start"
    >
      <ComponentsSidebar />
    </div>

    <!-- Center: Canvas workspace; the document body owns vertical scrolling. -->
    <CanvasWorkspace>
      {@render children()}
    </CanvasWorkspace>

    <!-- Right: Inspector sidebar -->
    <div
      class="sticky top-7 z-20 h-[calc(100dvh-1.75rem)] flex-none overflow-hidden self-start"
    >
      <InspectorSidebar />
    </div>
  </div>
</div>
