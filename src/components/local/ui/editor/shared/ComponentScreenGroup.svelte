<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    label,
    badge,
    description,
    count,
    children,
  }: {
    label: string;
    badge: string;
    description: string;
    count: number;
    children?: Snippet;
  } = $props();
</script>

<section class="space-y-4">
  <div class="flex items-end gap-3 border-b border-blender-border pb-2">
    <div class="min-w-0 flex-1">
      <p class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500">{badge}</p>
      <h2 class="text-base font-semibold text-zinc-100">{label}</h2>
      <p class="mt-0.5 text-xs text-zinc-500">{description}</p>
    </div>
    <span class="shrink-0 rounded-full border border-blender-border bg-blender-base px-2.5 py-0.5 text-[10px] font-medium text-zinc-500">{count}</span>
  </div>
  {@render children?.()}
</section>
