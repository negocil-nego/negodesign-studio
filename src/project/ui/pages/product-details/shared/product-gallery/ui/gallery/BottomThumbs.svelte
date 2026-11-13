<script lang="ts">
  import Icon from "../../../../../../atoms/icon/ui/Icon.svelte";
  import CardImage from "../../../../../../molecules/card/shared/card-image/ui/CardImage.svelte";
  import CardThumbnailVideo from "../../../../../../molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo.svelte";
  import CardVideo from "../../../../../../molecules/card/shared/card-video/ui/CardVideo.svelte";
  import type { ProductGalleryProps } from "../../data/types";

  type GalleryImage = NonNullable<ProductGalleryProps["galleryImages"]>[number];
  type Thumb = { imageProps?: GalleryImage; active?: boolean };

  let {
    currentImage,
    allThumbs = [],
    activeIdx = 0,
    badge,
    showArrows = false,
    videoProps,
    thumbnailVideoProps,
    className = "",
    onSelect,
    onPrev,
    onNext,
  }: {
    currentImage: GalleryImage;
    allThumbs: (Thumb | { imageProps: GalleryImage; active: boolean })[];
    activeIdx: number;
    badge?: string;
    showArrows?: boolean;
    videoProps?: ProductGalleryProps["videoProps"];
    thumbnailVideoProps?: ProductGalleryProps["thumbnailVideoProps"];
    className?: string;
    onSelect: (idx: number) => void;
    onPrev: () => void;
    onNext: () => void;
  } = $props();
</script>

<div class={`space-y-3.5 ${className}`}>
  <div class="relative aspect-[4/3.6] w-full overflow-hidden rounded-2xl border border-blender-border bg-blender-surface">
    {#if badge}
      <span class="absolute left-3.5 top-3.5 z-10 rounded-full border border-blue-500/40 bg-blue-500/15 px-3 py-1 text-xs font-semibold text-blue-300">
        {badge}
      </span>
    {/if}

    {#if thumbnailVideoProps}
      <CardThumbnailVideo {...thumbnailVideoProps} />
    {:else if videoProps}
      <CardVideo {...videoProps} controls={false} />
    {:else}
      <CardImage {...currentImage} className="h-full w-full object-cover" />
    {/if}

    {#if showArrows && allThumbs.length > 1}
      <button
        type="button"
        onclick={onPrev}
        aria-label="Imagem anterior"
        class="absolute left-3 top-1/2 z-10 flex size-9 -translate-y-1/2 items-center justify-center rounded-lg border border-blender-border bg-blender-card/90 text-zinc-200 shadow transition hover:border-blue-500 hover:text-white"
      >
        <Icon name="ChevronLeft" className="size-4" />
      </button>
      <button
        type="button"
        onclick={onNext}
        aria-label="Próxima imagem"
        class="absolute right-3 top-1/2 z-10 flex size-9 -translate-y-1/2 items-center justify-center rounded-lg border border-blender-border bg-blender-card/90 text-zinc-200 shadow transition hover:border-blue-500 hover:text-white"
      >
        <Icon name="ChevronRight" className="size-4" />
      </button>
    {/if}
  </div>

  {#if allThumbs.length > 0}
    <div class="grid grid-cols-4 gap-2.5 sm:grid-cols-5">
      {#each allThumbs as thumb, idx (idx)}
        <button
          type="button"
          onclick={() => onSelect(idx)}
          class={`relative aspect-square overflow-hidden rounded-xl border bg-blender-surface transition ${
            activeIdx === idx
              ? "border-blue-500 ring-2 ring-blue-500/30"
              : "border-blender-border hover:border-blue-500/50"
          }`}
        >
          {#if thumb.imageProps}
            <CardImage {...thumb.imageProps} className="h-full w-full object-cover" />
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>
