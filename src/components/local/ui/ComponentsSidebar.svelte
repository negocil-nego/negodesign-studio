<script lang="ts">
  type ComponentItem = { name: string; icon: string };
  type ComponentGroup = {
    label: string;
    items: ComponentItem[];
    open?: boolean;
  };

  let groups = $state<ComponentGroup[]>([
    {
      label: "Input",
      open: true,
      items: [
        { name: "Text", icon: "▣" },
        { name: "Email", icon: "⊞" },
        { name: "Phone", icon: "🃏" },
        { name: "Password", icon: "🃏" },
        { name: "Number", icon: "🃏" },
        { name: "Date", icon: "🃏" },
        { name: "Time", icon: "🃏" },
      ],
    },
    {
      label: "Controls",
      open: false,
      items: [
        { name: "Textarea", icon: "▭" },
        { name: "Select", icon: "⬽" },
        { name: "Toggle", icon: "⏻" },
        { name: "Slider", icon: "▬" },
        { name: "Rating", icon: "▬" },
      ],
    },
    {
      label: "Typography",
      open: false,
      items: [
        { name: "Heading", icon: "H" },
        { name: "Text", icon: "T" },
        { name: "Label", icon: "L" },
        { name: "Badge", icon: "●" },
      ],
    },
    {
      label: "Media",
      open: false,
      items: [
        { name: "Image", icon: "🖼" },
        { name: "Icon", icon: "★" },
        { name: "Avatar", icon: "👤" },
        { name: "Video", icon: "▶" },
      ],
    },
    {
      label: "Navigation",
      open: false,
      items: [
        { name: "Navbar", icon: "≡" },
        { name: "Sidebar", icon: "▐" },
        { name: "Tabs", icon: "⊟" },
        { name: "Breadcrumb", icon: "›" },
      ],
    },
  ]);

  let search = $state("");
  function toggle(g: ComponentGroup) {
    g.open = !g.open;
  }
  function filtered(items: ComponentItem[]) {
    if (!search.trim()) return items;
    return items.filter((i) =>
      i.name.toLowerCase().includes(search.toLowerCase()),
    );
  }
</script>

<aside
  class="blender-sidebar flex flex-col w-50 min-w-40 h-full overflow-hidden"
>
  <div class="blender-topbar px-3 border-b border-blender-border">
    <span
      class="text-[10px] font-semibold uppercase tracking-widest text-zinc-400"
      >Library</span
    >
    <div class="flex items-center gap-1">
      <button class="blender-icon-btn w-5 h-5 text-[10px]" title="Settings"
        >⚙</button
      >
    </div>
  </div>

  <div class="px-2 py-1.5 border-b border-blender-border">
    <input
      bind:value={search}
      placeholder="Search…"
      class="w-full bg-blender-base border border-blender-border rounded px-2 py-1 text-[11px]
             text-zinc-300 placeholder:text-zinc-600 focus:outline-none focus:border-blue-500/50 transition-colors"
    />
  </div>

  <div class="flex-1 overflow-y-auto">
    {#each groups as group}
      {@const vis = filtered(group.items)}
      {#if !search.trim() || vis.length}
        <div class="border-b border-blender-border/40">
          <button
            onclick={() => toggle(group)}
            class="w-full flex items-center justify-between px-3 py-1.5 text-[10px] font-semibold
                   uppercase tracking-widest text-zinc-500 hover:text-zinc-200 hover:bg-blender-hover transition-colors"
          >
            <span>{group.label}</span>
            <span
              class="text-[8px]"
              style="transform: rotate({group.open
                ? 0
                : -90}deg); display:inline-block; transition: transform .2s"
              >▼</span
            >
          </button>
          {#if group.open || search.trim()}
            <ul class="pb-1">
              {#each vis as item}
                <li>
                  <button
                    class="blender-item w-full text-left gap-2 px-3 py-1"
                    draggable="true"
                    title={item.name}
                  >
                    <span
                      class="text-[11px] w-4 text-center text-blue-400 shrink-0"
                      >{item.icon}</span
                    >
                    <span class="text-[11px] truncate">{item.name}</span>
                  </button>
                </li>
              {/each}
            </ul>
          {/if}
        </div>
      {/if}
    {/each}
  </div>
</aside>
