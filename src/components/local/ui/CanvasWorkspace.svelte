<script lang="ts">
  import type { Snippet } from "svelte";
  let { children }: { children?: Snippet } = $props();

  let zoom = $state(100);
  let isDragOver = $state(false);
  let selectedTool = $state<"select" | "move" | "pan">("select");

  const tools = [
    { id: "select", label: "Select", icon: "↖" },
    { id: "move", label: "Move", icon: "✥" },
    { id: "pan", label: "Pan", icon: "✋" },
  ] as const;

  function onDragOver(e: DragEvent) {
    e.preventDefault();
    isDragOver = true;
  }
  function onDragLeave() {
    isDragOver = false;
  }
  function onDrop(e: DragEvent) {
    e.preventDefault();
    isDragOver = false;
  }

  function zoomIn() {
    zoom = Math.min(zoom + 10, 400);
  }
  function zoomOut() {
    zoom = Math.max(zoom - 10, 10);
  }
  function resetZoom() {
    zoom = 100;
  }
</script>

<div class="blender-workspace relative flex flex-col overflow-hidden p-0">
  <!-- Toolbar -->
  <div
    class="absolute top-3 left-1/2 -translate-x-1/2 z-10 flex items-center gap-1
              bg-blender-card border border-blender-border rounded-lg px-2 py-1.5 shadow-xl"
  >
    {#each tools as tool}
      <button
        onclick={() => (selectedTool = tool.id)}
        title={tool.label}
        class="blender-icon-btn w-7 h-7 text-sm
               {selectedTool === tool.id
          ? 'bg-blender-base! border-blender-border! text-blue-400!'
          : ''}">{tool.icon}</button
      >
    {/each}
    <div class="w-px h-5 bg-blender-border mx-1"></div>
    <button
      onclick={zoomOut}
      class="blender-icon-btn w-7 h-7 text-sm"
      title="Zoom out">−</button
    >
    <button
      onclick={resetZoom}
      class="min-w-11.5 text-[11px] text-zinc-400 hover:text-zinc-200
            bg-blender-base border border-blender-border rounded px-1.5 py-0.5 transition-colors"
      title="Reset zoom"
    >
      {zoom}%
    </button>
    <button
      onclick={zoomIn}
      class="blender-icon-btn w-7 h-7 text-sm"
      title="Zoom in">+</button
    >
  </div>

  <!-- Canvas area -->
  <div
    role="region"
    aria-label="Design canvas"
    class="flex-1 w-full h-full flex items-center justify-center relative overflow-hidden
           bg-blender-base transition-colors {isDragOver
      ? 'bg-blue-900/10'
      : ''}"
    style="background-image: radial-gradient(circle, #2d3036 1px, transparent 1px);
           background-size: 24px 24px;"
    ondragover={onDragOver}
    ondragleave={onDragLeave}
    ondrop={onDrop}
  >
    <!-- Artboard -->
    <div
      class="relative shadow-2xl border border-blender-border/60 rounded bg-white
             transition-all duration-200 overflow-hidden"
      style="width: {Math.round((880 * zoom) / 100)}px; height: {Math.round(
        (560 * zoom) / 100,
      )}px;"
    >
      {#if children}
        {@render children()}
      {:else}
        <div
          class="absolute inset-0 flex flex-col items-center justify-center gap-3
                    border-2 border-dashed border-zinc-300/20 rounded text-zinc-400/40"
        >
          <div class="text-3xl opacity-30">⊞</div>
          <span class="text-[12px]">Drag Library items here…</span>
        </div>
      {/if}
    </div>

    <!-- Drop highlight overlay -->
    {#if isDragOver}
      <div
        class="absolute inset-0 border-2 border-blue-500/40 bg-blue-500/5 pointer-events-none rounded"
      ></div>
    {/if}
  </div>

  <!-- Status bar -->
  <div class="blender-footer shrink-0">
    <span class="text-[10px] text-zinc-600">No Selection</span>
    <span class="text-[10px] text-zinc-600">{zoom}%</span>
  </div>
</div>
