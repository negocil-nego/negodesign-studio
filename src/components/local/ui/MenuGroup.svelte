<script lang="ts">
  import { type IconNamed, getIconUrl } from "../../../hooks/icon.svelte";

  type Props = {
    name: string;
    items: { name: string; path: string; icon: IconNamed }[];
  };

  let { name, items }: Props = $props();
</script>

<div class="space-y-3 select-none font-commissioner">
  <div class="blender-header px-2 py-1 text-zinc-400">
    <span class="text-[9px] text-zinc-400">▼</span>
    <span class="text-zinc-300">{name}</span>
  </div>

  {#if items.length > 0}
    <ul class="space-y-2">
      {#each items as item}
        <li>
          <a href={item.path} class="blender-item group">
            <img
              src={getIconUrl(item.icon)}
              alt={item.name}
              class="w-4 h-4 brightness-0 invert opacity-70 group-hover:opacity-100 transition-opacity"
            />
            <span class="truncate">{item.name}</span>
          </a>
        </li>
      {/each}
    </ul>
  {:else}
    <div
      class="px-3 py-2.5 border border-dashed border-blender-border rounded bg-blender-base/60 text-center"
    >
      <span class="text-[11px] text-zinc-400">Nenhum projecto recente</span>
    </div>
  {/if}
</div>
