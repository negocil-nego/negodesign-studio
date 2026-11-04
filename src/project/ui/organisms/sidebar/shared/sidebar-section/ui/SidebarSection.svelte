<script lang="ts">
  import SidebarItem from "../../sidebar-item/ui/SidebarItem.svelte";
  import type { SidebarSectionProps } from "../../../data/types";

  let {
    section,
    compact = false,
    dark = false,
    accent = "blue",
  }: {
    section: SidebarSectionProps;
    compact?: boolean;
    dark?: boolean;
    accent?: string;
  } = $props();
</script>

<section class="space-y-1.5">
  {#if section.title && !compact}
    <h3
      class={`px-3 pb-1 pt-3 text-[10px] font-semibold uppercase tracking-[0.14em] ${dark ? "text-zinc-500" : "text-zinc-400"}`}
    >
      {section.title}
    </h3>
  {/if}

  <div class="space-y-1">
    {#each section.items as item (item.label)}
      <SidebarItem {item} {compact} {dark} {accent} />
    {/each}
  </div>
</section>
