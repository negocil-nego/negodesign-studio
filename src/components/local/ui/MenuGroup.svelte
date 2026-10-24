<script lang="ts">
  import { type IconNamed, getIconClass } from "../../../hooks/icon.svelte";
  import { StyleComponent } from "../data/style.component";

  type Props = {
    name: string;
    items: { name: string; path: string; icon: IconNamed }[];
  };

  let { name, items }: Props = $props();
</script>

<div class="space-y-3 select-none font-commissioner">
  <div class="{StyleComponent.layout.header} px-2 py-1 text-zinc-400">
    <span class="text-[9px] text-zinc-400">▼</span>
    <span class="text-zinc-300">{name}</span>
  </div>

  {#if items.length > 0}
    <ul class="space-y-2">
      {#each items as item}
        <li>
          <a href={item.path} class="{StyleComponent.layout.item} group">
            <i class={`${getIconClass(item.icon)} w-5 h-5 text-base opacity-70 group-hover:opacity-100 transition-opacity`} aria-label={item.name}></i>
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
