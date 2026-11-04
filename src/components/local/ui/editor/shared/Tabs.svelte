<script lang="ts">
  import { tick } from "svelte";

  export type TabItem = {
    id: string;
    label: string;
    count?: number;
  };

  let {
    items,
    value,
    panelId,
    idPrefix = "tab",
    onselect,
  }: {
    items: TabItem[];
    value: string;
    panelId: string;
    idPrefix?: string;
    onselect: (value: string) => void;
  } = $props();

  async function handleKeydown(event: KeyboardEvent, index: number) {
    const targetIndex = {
      ArrowRight: (index + 1) % items.length,
      ArrowLeft: (index - 1 + items.length) % items.length,
      Home: 0,
      End: items.length - 1,
    }[event.key];

    if (targetIndex === undefined) return;

    event.preventDefault();
    const target = items[targetIndex];
    onselect(target.id);
    await tick();
    document.getElementById(`${idPrefix}-${target.id}`)?.focus();
  }
</script>

<div
  class="flex min-w-0 gap-1 overflow-x-auto border-b border-blender-border"
  role="tablist"
  aria-label="Grupos de componentes"
>
  {#each items as item, index (item.id)}
    <button
      id={`${idPrefix}-${item.id}`}
      type="button"
      role="tab"
      aria-selected={value === item.id}
      aria-controls={panelId}
      tabindex={value === item.id ? 0 : -1}
      onclick={() => onselect(item.id)}
      onkeydown={(event) => handleKeydown(event, index)}
      class={`inline-flex shrink-0 items-center gap-2 border-b-2 px-3 py-2.5 text-xs font-medium transition-colors ${value === item.id ? "border-blue-500 text-blue-400" : "border-transparent text-zinc-500 hover:text-zinc-200"}`}
    >
      <span>{item.label}</span>
      {#if item.count !== undefined}
        <span class="rounded-full bg-blender-hover px-1.5 py-0.5 text-[10px] text-zinc-400">
          {item.count}
        </span>
      {/if}
    </button>
  {/each}
</div>
