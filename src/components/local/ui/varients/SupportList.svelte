<script lang="ts">
  import type { SupportItem } from "../../data/type";

  let { items }: { items: SupportItem[] } = $props();
</script>

<div class="flex flex-wrap gap-3">
  {#each items as item (item.label)}
    <div
      title={item.label}
      class="group relative border border-slate-200 bg-white hover:bg-slate-100 hover:border-blue-500/60 rounded-full p-2 flex justify-center items-center transition-all duration-200 hover:scale-110 shadow-sm cursor-pointer"
    >
      <img src={item.image} alt={item.label} class="w-6 h-6 object-contain" />
      <span class="sr-only">{item.label}</span>
    </div>
  {/each}
</div>
