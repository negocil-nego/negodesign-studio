<script lang="ts">
  import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
  import CardImage from "../../../../../molecules/card/shared/card-image/ui/CardImage.svelte";
  import CardMinFile from "../../../../../molecules/card/shared/card-min-file/ui/CardMinFile.svelte";
  import CardThumbnailVideo from "../../../../../molecules/card/shared/card-thumbnail-video/ui/CardThumbnailVideo.svelte";
  import CardVideo from "../../../../../molecules/card/shared/card-video/ui/CardVideo.svelte";
  import type { ProductGalleryProps } from "../data/types";

  let {
    layout = "bottom-thumbs",
    badge,
    showArrows = false,
    imageProps,
    videoProps,
    thumbnailVideoProps,
    minFileProps = [],
    galleryImages = [],
    className = "",
  }: ProductGalleryProps = $props();

  let activeIdx = $state(0);

  const allThumbs = $derived(
    minFileProps.length > 0
      ? minFileProps
      : galleryImages.map((img, i) => ({
          imageProps: img,
          active: i === activeIdx,
        })),
  );

  const currentImage = $derived(
    allThumbs[activeIdx]?.imageProps ??
      imageProps ?? {
        src: "/img/ux-01.jpg",
        alt: "Product preview",
      },
  );

  function selectIndex(idx: number) {
    activeIdx = idx;
  }

  function prevSlide() {
    if (allThumbs.length === 0) return;
    activeIdx = (activeIdx - 1 + allThumbs.length) % allThumbs.length;
  }

  function nextSlide() {
    if (allThumbs.length === 0) return;
    activeIdx = (activeIdx + 1) % allThumbs.length;
  }
</script>

{#if layout === "grid-gallery"}
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
              onclick={() => selectIndex(idx)}
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
{:else if layout === "left-thumbs"}
  <div class={`grid grid-cols-1 gap-4 sm:grid-cols-[5.5rem_minmax(0,1fr)] ${className}`}>
    {#if allThumbs.length > 0}
      <div class="order-2 flex gap-2.5 overflow-x-auto sm:order-1 sm:flex-col">
        {#each allThumbs as thumb, idx (idx)}
          <div class="size-20 shrink-0">
            <CardMinFile
              imageProps={thumb.imageProps}
              active={activeIdx === idx}
              onClick={() => selectIndex(idx)}
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
          onclick={prevSlide}
          aria-label="Imagem anterior"
          class="absolute left-3 top-1/2 z-10 flex size-9 -translate-y-1/2 items-center justify-center rounded-lg border border-blender-border bg-blender-card/90 text-zinc-200 shadow transition hover:border-blue-500 hover:text-white"
        >
          <Icon name="ChevronLeft" className="size-4" />
        </button>
        <button
          type="button"
          onclick={nextSlide}
          aria-label="Próxima imagem"
          class="absolute right-3 top-1/2 z-10 flex size-9 -translate-y-1/2 items-center justify-center rounded-lg border border-blender-border bg-blender-card/90 text-zinc-200 shadow transition hover:border-blue-500 hover:text-white"
        >
          <Icon name="ChevronRight" className="size-4" />
        </button>
      {/if}
    </div>
  </div>
{:else}
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
          onclick={prevSlide}
          aria-label="Imagem anterior"
          class="absolute left-3 top-1/2 z-10 flex size-9 -translate-y-1/2 items-center justify-center rounded-lg border border-blender-border bg-blender-card/90 text-zinc-200 shadow transition hover:border-blue-500 hover:text-white"
        >
          <Icon name="ChevronLeft" className="size-4" />
        </button>
        <button
          type="button"
          onclick={nextSlide}
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
            onclick={() => selectIndex(idx)}
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
{/if}
