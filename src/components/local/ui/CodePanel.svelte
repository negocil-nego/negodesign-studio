<script lang="ts">
  let selectedElement = $state('Box');
  let code = $state(`<!-- Box component -->
<div class="box-container">
  <!-- Drag components here -->
</div>`);

  let copied = $state(false);

  async function copyCode() {
    await navigator.clipboard.writeText(code);
    copied = true;
    setTimeout(() => copied = false, 1500);
  }

  function formatCode() {
    // basic formatting hint
    code = code.replace(/></g, '>\n<').replace(/^\s*\n/gm, '');
  }
</script>

<div class="flex flex-col h-full overflow-hidden">
  <!-- Toolbar -->
  <div class="flex items-center gap-1 px-2 py-1.5 border-b border-blender-border/40 bg-blender-base/60 shrink-0">
    <span class="text-[10px] text-zinc-500 flex-1 truncate">Code : {selectedElement}</span>
    <button
      onclick={formatCode}
      class="blender-icon-btn w-5 h-5 text-[9px]" title="Format"
    >⌥</button>
    <button
      onclick={copyCode}
      class="blender-icon-btn w-5 h-5 text-[9px] {copied ? '!text-green-400 !border-green-400/40' : ''}"
      title="Copy"
    >{copied ? '✓' : '⎘'}</button>
  </div>

  <!-- Syntax hint banner -->
  <div class="px-2 py-1 bg-blue-500/5 border-b border-blue-500/10 shrink-0">
    <span class="text-[9px] text-blue-400/70">Svelte 5 · Component code</span>
  </div>

  <!-- Code textarea -->
  <div class="flex-1 overflow-hidden relative">
    <textarea
      bind:value={code}
      spellcheck="false"
      class="absolute inset-0 w-full h-full bg-blender-base text-[11px] text-emerald-300 font-mono
             px-3 py-2 resize-none focus:outline-none leading-relaxed border-0"
    ></textarea>
  </div>

  <!-- Line count -->
  <div class="flex items-center justify-between px-2 py-1 border-t border-blender-border/40 bg-blender-base/80 shrink-0">
    <span class="text-[9px] text-zinc-600 font-mono">
      {code.split('\n').length} lines
    </span>
    <span class="text-[9px] text-zinc-700">UTF-8</span>
  </div>
</div>
