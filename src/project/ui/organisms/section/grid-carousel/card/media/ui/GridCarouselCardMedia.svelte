<script lang="ts">
  import type { GridCarouselCardMediaProps } from "../data/types";
  import CardMedia from "../../../../../../molecules/card/media/ui/CardMedia.svelte";
  import SectionGridCarousel from "../../../ui/SectionGridCarousel.svelte";

  let { items, sectionGridCarouselProps }: GridCarouselCardMediaProps = $props();
</script>

<SectionGridCarousel {...sectionGridCarouselProps}>
  {#each items as item}
    <CardMedia {...item} />
  {/each}
</SectionGridCarousel>
