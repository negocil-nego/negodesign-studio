<script lang="ts">
  import CardImage from "../../../../../../molecules/card/shared/card-image/ui/CardImage.svelte";
  import type { ProductGalleryProps } from "../../data/types";

  type GalleryImage = NonNullable<ProductGalleryProps["galleryImages"]>[number];
  type Thumb = { imageProps?: GalleryImage; active?: boolean };

  let {
    currentImage,
    allThumbs = [],
    activeIdx = 0,
    badge,
    className = "",
    onSelect,
  }: {
    currentImage: GalleryImage;
    allThumbs: Thumb[];
    activeIdx: number;
    badge?: string;
    className?: string;
    onSelect: (idx: number) => void;
  } = $props();
</script>

<div class={`space-y-4 ${className}`}>
  <div class="relative aspect-[4/3] w-full overflow-hidden rounded-2xl border border-blender-border bg-blender-surface">
    {#if badge}
      <span class="absolute left-3 top-3 z-10 rounded-full border border-blue-500/40 bg-blue-500/15 px-3 py-1 text-xs font-semibold text-blue-300">
        {badge}
      </span>
    {/if}
    <CardImage {...currentImage} className="h-full w-full object-cover" />
  </div>

  {#if allThumbs.length > 0}
    <div class="grid grid-cols-2 gap-3 sm:grid-cols-2">
      {#each allThumbs as thumb, idx (idx)}
        {#if thumb.imageProps}
          <button
            type="button"
            onclick={() => onSelect(idx)}
            class={`group relative aspect-square overflow-hidden rounded-xl border bg-blender-surface transition ${
              activeIdx === idx
                ? "border-blue-500 ring-2 ring-blue-500/30"
                : "border-blender-border hover:border-blue-500/50"
            }`}
          >
            <CardImage {...thumb.imageProps} className="h-full w-full object-cover transition group-hover:scale-105" />
          </button>
        {/if}
      {/each}
    </div>
  {/if}
</div>
