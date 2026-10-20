<script lang="ts">
  import PropertiesPanel from './PropertiesPanel.svelte';
  import LayoutPanel from './LayoutPanel.svelte';
  import CodePanel from './CodePanel.svelte';

  type Tab = 'properties' | 'layout' | 'code';
  let activeTab = $state<Tab>('properties');
  let selectedElement = $state('Accordion');

  const tabs: { id: Tab; label: string }[] = [
    { id: 'properties', label: 'Properties' },
    { id: 'layout', label: 'Layout' },
    { id: 'code', label: 'Code' },
  ];
</script>

<aside class="blender-sidebar flex flex-col w-[260px] min-w-[220px] h-full overflow-hidden border-l border-blender-border">
  <!-- Header -->
  <div class="blender-topbar px-3 border-b border-blender-border">
    <span class="text-[10px] font-semibold uppercase tracking-widest text-zinc-400">Inspector</span>
    <div class="flex items-center gap-1">
      <button class="blender-icon-btn w-5 h-5 text-[10px]" title="Search">⌕</button>
      <button class="blender-icon-btn w-5 h-5 text-[10px]" title="Settings">⚙</button>
    </div>
  </div>

  <!-- Tab navigation (as collapsible rows like SceneBuilder) -->
  <div class="flex flex-col overflow-hidden flex-1">
    {#each tabs as tab}
      <div class="flex flex-col {activeTab === tab.id ? 'flex-1' : ''} overflow-hidden border-b border-blender-border/40">
        <!-- Tab header / toggle -->
        <button
          onclick={() => activeTab = tab.id}
          class="w-full flex items-center justify-between px-3 py-1.5 text-[10px] font-semibold
                 uppercase tracking-wider transition-colors shrink-0
                 {activeTab === tab.id
                   ? 'bg-blender-hover text-zinc-200 border-l-2 border-blue-500'
                   : 'text-zinc-500 hover:text-zinc-300 hover:bg-blender-hover/50'}"
        >
          <span>{tab.label} : {selectedElement}</span>
          <span class="text-[8px]" style="transform: rotate({activeTab === tab.id ? 0 : -90}deg); display:inline-block; transition: transform .2s">▼</span>
        </button>

        <!-- Tab content -->
        {#if activeTab === tab.id}
          <div class="flex-1 overflow-y-auto">
            {#if tab.id === 'properties'}
              <PropertiesPanel />
            {:else if tab.id === 'layout'}
              <LayoutPanel />
            {:else}
              <CodePanel />
            {/if}
          </div>
        {/if}
      </div>
    {/each}
  </div>
</aside>

