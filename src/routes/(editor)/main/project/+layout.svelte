<script lang="ts">
  import type { Snippet } from "svelte";
  import ComponentsSidebar from "../../../../components/local/ui/ComponentsSidebar.svelte";
  import InspectorSidebar from "../../../../components/local/ui/InspectorSidebar.svelte";
  import CanvasWorkspace from "../../../../components/local/ui/CanvasWorkspace.svelte";
  import MenuNavigation from "../../../../components/local/ui/MenuNavigation.svelte";

  let { children }: { children: Snippet } = $props();
</script>

<div class="blender-app flex flex-col">
  <MenuNavigation />
  <!-- Main editor area -->
  <div class="flex flex-1 overflow-hidden">
    <!-- Left: Components Library sidebar -->
    <ComponentsSidebar />

    <!-- Center: Canvas workspace -->
    <CanvasWorkspace>
      {@render children()}
    </CanvasWorkspace>

    <!-- Right: Inspector sidebar -->
    <InspectorSidebar />
  </div>
</div>
