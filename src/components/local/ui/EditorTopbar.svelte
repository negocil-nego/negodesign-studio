<script lang="ts">
  type Props = {
    projectName?: string;
    onSave?: () => void;
    onPreview?: () => void;
    onUndo?: () => void;
    onRedo?: () => void;
  };

  let { projectName = 'Untitled Project', onSave, onPreview, onUndo, onRedo }: Props = $props();
  let saved = $state(false);

  function handleSave() {
    onSave?.();
    saved = true;
    setTimeout(() => saved = false, 2000);
  }
</script>

<header class="blender-topbar border-b border-blender-border shrink-0 px-3 gap-3">
  <!-- Logo & project name -->
  <div class="flex items-center gap-2.5">
    <div class="w-6 h-6 rounded bg-gradient-to-br from-blue-500 to-violet-600 flex items-center justify-center text-[11px] font-bold shadow-lg">N</div>
    <span class="text-[12px] font-semibold text-zinc-200 max-w-[180px] truncate">{projectName}</span>
    <span class="text-zinc-700">·</span>
    <span class="text-[10px] text-zinc-500">{saved ? '✓ Saved' : 'Unsaved'}</span>
  </div>

  <!-- Center actions -->
  <div class="flex items-center gap-1 mx-auto">
    <button onclick={onUndo} class="blender-icon-btn w-7 h-7 text-[13px]" title="Undo">↩</button>
    <button onclick={onRedo} class="blender-icon-btn w-7 h-7 text-[13px]" title="Redo">↪</button>
    <div class="w-px h-5 bg-blender-border mx-1"></div>
    <button onclick={handleSave}
      class="flex items-center gap-1.5 px-3 py-1 rounded text-[11px] font-semibold bg-blue-600 hover:bg-blue-500
             text-white transition-colors border border-blue-500/60 shadow"
    >
      <span class="text-[10px]">💾</span> Save
    </button>
    <button onclick={onPreview}
      class="flex items-center gap-1.5 px-3 py-1 rounded text-[11px] font-semibold
             bg-blender-card hover:bg-blender-hover border border-blender-border text-zinc-300 transition-colors"
    >
      <span class="text-[10px]">▶</span> Preview
    </button>
  </div>

  <!-- Right -->
  <div class="flex items-center gap-1 ml-auto">
    <button class="blender-icon-btn w-7 h-7 text-[12px]" title="Settings">⚙</button>
    <button class="blender-icon-btn w-7 h-7 text-[12px]" title="Help">?</button>
  </div>
</header>
