<script lang="ts">
  import { componentNavigationGroups } from "../data/components";

  let componentsOpen = $state(false);
</script>

<nav class="flex h-fit w-fit flex-none items-center gap-1 self-start font-commissioner text-xs" aria-label="Menu principal">
  <a href="/" class="rounded px-2.5 py-1.5 text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100">
    Início
  </a>
  <a href="/main/project" class="rounded px-2.5 py-1.5 text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100">
    Projecto
  </a>

  <div class="relative">
    <button
      type="button"
      class="flex items-center gap-1 rounded px-2.5 py-1.5 text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"
      aria-expanded={componentsOpen}
      onclick={() => (componentsOpen = !componentsOpen)}
    >
      Componentes
      <span class="text-[10px] transition-transform" class:rotate-180={componentsOpen}>⌄</span>
    </button>

    {#if componentsOpen}
      <div class="absolute left-0 top-full z-50 max-h-[70vh] w-64 overflow-y-auto rounded-md border border-blender-border bg-blender-card p-1.5 shadow-xl">
        {#each componentNavigationGroups as group (group.label)}
          {@const documented = group.items.filter((item) => item.href)}
          {#if documented.length}
            <details class="group" open={group.label === "Input"}>
              <summary class="cursor-pointer rounded px-2 py-1.5 text-[10px] font-semibold uppercase tracking-wider text-zinc-500 transition-colors hover:bg-blender-hover hover:text-zinc-200">
                {group.label}
              </summary>
              <ul class="pb-1">
                {#each documented as item (item.id)}
                  <li>
                    <a
                      href={item.href}
                      class="flex items-center gap-2 rounded px-2 py-1.5 text-xs text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"
                      title={item.label ?? item.name}
                      onclick={() => (componentsOpen = false)}
                    >
                      <span class="truncate">{item.label ?? item.name}</span>
                      <span class="ml-auto text-[10px] text-zinc-500">{item.group}</span>
                    </a>
                  </li>
                {/each}
              </ul>
            </details>
          {/if}
        {/each}
      </div>
    {/if}
  </div>
</nav>
