<script lang="ts">
  import { onMount } from "svelte";
  import { IconNamed, getIconUrl } from "../../../hooks/icon.svelte";

  type ComponentItem = { name: string; icon: IconNamed };
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
        { name: "Text", icon: IconNamed.Layout04 },
        { name: "Email", icon: IconNamed.DocumentValidation },
        { name: "Phone", icon: IconNamed.Home },
        { name: "Password", icon: IconNamed.Security },
        { name: "Number", icon: IconNamed.CheckmarkSquare01 },
        { name: "Date", icon: IconNamed.Languages },
        { name: "Time", icon: IconNamed.Settings01 },
      ],
    },
    {
      label: "Controls",
      open: false,
      items: [
        { name: "Textarea", icon: IconNamed.Layout04 },
        { name: "Select", icon: IconNamed.CheckmarkSquare01 },
        { name: "Toggle", icon: IconNamed.Settings01 },
        { name: "Slider", icon: IconNamed.Languages },
        { name: "Rating", icon: IconNamed.HelpSquare },
      ],
    },
    {
      label: "Typography",
      open: false,
      items: [
        { name: "Heading", icon: IconNamed.DocumentValidation },
        { name: "Text", icon: IconNamed.Layout04 },
        { name: "Label", icon: IconNamed.CheckmarkSquare01 },
        { name: "Badge", icon: IconNamed.HelpSquare },
      ],
    },
    {
      label: "Media",
      open: false,
      items: [
        { name: "Image", icon: IconNamed.Layout04 },
        { name: "Icon", icon: IconNamed.HelpSquare },
        { name: "Avatar", icon: IconNamed.Home },
        { name: "Video", icon: IconNamed.DocumentValidation },
      ],
    },
    {
      label: "Navigation",
      open: false,
      items: [
        { name: "Navbar", icon: IconNamed.Layout04 },
        { name: "Sidebar", icon: IconNamed.FolderAdd },
        { name: "Tabs", icon: IconNamed.Languages },
        { name: "Breadcrumb", icon: IconNamed.DocumentValidation },
      ],
    },
  ]);

  let search = $state("");
  let theme = $state<"light" | "dark">("light");

  onMount(() => {
    const savedTheme = localStorage.getItem("negodesign-theme");
    theme = savedTheme === "dark" ? "dark" : "light";
    document.documentElement.dataset.theme = theme;
  });

  function toggleTheme() {
    theme = theme === "light" ? "dark" : "light";
    document.documentElement.dataset.theme = theme;
    localStorage.setItem("negodesign-theme", theme);
  }
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
      class="text-xs font-semibold uppercase tracking-widest text-zinc-500"
      >Library</span
    >
    <div class="flex items-center gap-1">
      <button
        class="blender-icon-btn h-7 w-7 text-base"
        type="button"
        onclick={toggleTheme}
        title={theme === "light" ? "Ativar modo escuro" : "Ativar modo claro"}
        aria-label={theme === "light" ? "Ativar modo escuro" : "Ativar modo claro"}
      >{theme === "light" ? "☾" : "☀"}</button>
      <button class="blender-icon-btn h-7 w-7" title="Settings">
        <img
          src={getIconUrl(IconNamed.Settings01)}
          alt=""
          class="h-4 w-4 opacity-75"
        />
      </button>
    </div>
  </div>

  <div class="px-2 py-1.5 border-b border-blender-border">
    <input
      bind:value={search}
      placeholder="Search…"
      class="w-full bg-blender-base border border-blender-border rounded px-2.5 py-2 text-sm
             text-zinc-300 placeholder:text-zinc-500 focus:outline-none focus:border-blue-500/50 transition-colors"
    />
  </div>

  <div class="flex-1 overflow-y-auto">
    {#each groups as group}
      {@const vis = filtered(group.items)}
      {#if !search.trim() || vis.length}
        <div class="border-b border-blender-border/40">
          <button
            onclick={() => toggle(group)}
            class="w-full flex items-center justify-between px-3 py-2 text-xs font-semibold
                   uppercase tracking-widest text-zinc-500 hover:text-slate-900 hover:bg-blender-hover transition-colors"
          >
            <span>{group.label}</span>
            <span
              class="text-[10px]"
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
                    class="blender-item w-full text-left gap-2 px-3 py-2"
                    draggable="true"
                    title={item.name}
                  >
                    <img
                      src={getIconUrl(item.icon)}
                      alt=""
                      class="h-5 w-5 opacity-75 shrink-0"
                    />
                    <span class="text-sm truncate">{item.name}</span>
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

