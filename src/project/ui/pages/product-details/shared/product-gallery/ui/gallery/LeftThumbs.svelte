<script lang="ts">
  import Icon from "../../../../../../atoms/icon/ui/Icon.svelte";
  import CardImage from "../../../../../../molecules/card/shared/card-image/ui/CardImage.svelte";
  import CardMinFile from "../../../../../../molecules/card/shared/card-min-file/ui/CardMinFile.svelte";
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
    allThumbs: Thumb[];
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

<div class={`grid grid-cols-1 gap-4 sm:grid-cols-[5.5rem_minmax(0,1fr)] ${className}`}>
  {#if allThumbs.length > 0}
    <div class="order-2 flex gap-2.5 overflow-x-auto sm:order-1 sm:flex-col">
      {#each allThumbs as thumb, idx (idx)}
        <div class="size-20 shrink-0">
          <CardMinFile
            imageProps={thumb.imageProps}
            active={activeIdx === idx}
            onClick={() => onSelect(idx)}
          />
        </div>
      {/each}
    </div>
  {/if}

  <div class="order-1 relative aspect-[4/4] w-full overflow-hidden rounded-2xl border border-blender-border bg-blender-surface sm:order-2">
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
</div>
