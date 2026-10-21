<script lang="ts">
  import { IconNamed, getIconUrl } from "../../../hooks/icon.svelte";
  import PropertiesPanel from "./PropertiesPanel.svelte";
  import LayoutPanel from "./LayoutPanel.svelte";
  import CodePanel from "./CodePanel.svelte";

  type Tab = "properties" | "layout" | "code";
  let activeTab = $state<Tab>("properties");
  let selectedElement = $state("InputText");

  const tabs: { id: Tab; label: string }[] = [
    { id: "properties", label: "Properties" },
    { id: "layout", label: "Layout" },
    { id: "code", label: "Code" },
  ];
</script>

<aside
  class="blender-sidebar flex h-full w-70 min-w-60 flex-col overflow-hidden border-l border-blender-border"
>
  <div class="blender-topbar px-3">
    <span class="text-xs font-semibold uppercase tracking-widest text-zinc-500"
      >Inspector</span
    >
    <div class="flex items-center gap-1.5">
      <button
        class="blender-icon-btn h-7 w-7 text-lg"
        title="Search"
        aria-label="Search">⌕</button
      >
      <button
        class="blender-icon-btn h-7 w-7"
        title="Settings"
        aria-label="Settings"
      >
        <img
          src={getIconUrl(IconNamed.Settings01)}
          alt=""
          class="h-4 w-4 opacity-75"
        />
      </button>
    </div>
  </div>

  <div class="flex flex-1 flex-col overflow-hidden">
    {#each tabs as tab}
      <div
        class="flex flex-col overflow-hidden border-b border-blender-border/40 {activeTab ===
        tab.id
          ? 'flex-1'
          : ''}"
      >
        <button
          type="button"
          onclick={() => (activeTab = tab.id)}
          class="flex w-full shrink-0 items-center justify-between px-3 py-2 text-xs font-semibold uppercase tracking-wider transition-colors
            {activeTab === tab.id
            ? 'border-l-2 border-blue-500 bg-blender-hover text-zinc-800'
            : 'text-zinc-500 hover:bg-blender-hover/50 hover:text-zinc-300'}"
        >
          <span>{tab.label} : {selectedElement}</span>
          <span
            class="text-[10px]"
            style="transform: rotate({activeTab === tab.id
              ? 0
              : -90}deg); display:inline-block; transition: transform .2s"
            >▼</span
          >
        </button>

        {#if activeTab === tab.id}
          <div class="min-h-0 flex-1 overflow-y-auto">
            {#if tab.id === "properties"}
              <PropertiesPanel />
            {:else if tab.id === "layout"}
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
