<script lang="ts">
  import type { Snippet } from "svelte";
  import ComponentsSidebar from "../../../../components/local/ui/ComponentsSidebar.svelte";
  import InspectorSidebar from "../../../../components/local/ui/InspectorSidebar.svelte";
  import CanvasWorkspace from "../../../../components/local/ui/CanvasWorkspace.svelte";

  let { children }: { children: Snippet } = $props();
</script>

<div class="flex min-h-0 flex-1 flex-col overflow-hidden bg-blender-base">
  <!-- Main editor area -->
  <div class="flex min-h-0 flex-1 items-stretch overflow-hidden">
    <!-- Left: Components Library sidebar -->
    <div
      class="z-20 h-full flex-none overflow-hidden"
    >
      <ComponentsSidebar />
    </div>

    <!-- Center: Canvas workspace owns vertical scrolling. -->
    <CanvasWorkspace>
      {@render children()}
    </CanvasWorkspace>

    <!-- Right: Inspector sidebar -->
    <div
      class="z-20 h-full flex-none overflow-hidden"
    >
      <InspectorSidebar />
    </div>
  </div>
</div>
