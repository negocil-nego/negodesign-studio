<script lang="ts">
  import type { ProductGalleryProps } from "../data/types";
  import BottomThumbs from "./gallery/BottomThumbs.svelte";
  import GridGallery from "./gallery/GridGallery.svelte";
  import LeftThumbs from "./gallery/LeftThumbs.svelte";

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
  <GridGallery
    {currentImage}
    {allThumbs}
    {activeIdx}
    {badge}
    {className}
    onSelect={selectIndex}
  />
{:else if layout === "left-thumbs"}
  <LeftThumbs
    {currentImage}
    {allThumbs}
    {activeIdx}
    {badge}
    {showArrows}
    {videoProps}
    {thumbnailVideoProps}
    {className}
    onSelect={selectIndex}
    onPrev={prevSlide}
    onNext={nextSlide}
  />
{:else}
  <BottomThumbs
    {currentImage}
    {allThumbs}
    {activeIdx}
    {badge}
    {showArrows}
    {videoProps}
    {thumbnailVideoProps}
    {className}
    onSelect={selectIndex}
    onPrev={prevSlide}
    onNext={nextSlide}
  />
{/if}
