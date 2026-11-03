<script lang="ts">
  import type { GridCarouselCardProductProps } from "../data/types";
  import CardProduct from "../../../../../../molecules/card/product/ui/CardProduct.svelte";
  import SectionGridCarousel from "../../../ui/SectionGridCarousel.svelte";

  let { items, sectionGridCarouselProps }: GridCarouselCardProductProps = $props();
</script>

<SectionGridCarousel {...sectionGridCarouselProps}>
  {#each items as item}
    <CardProduct {...item} />
  {/each}
</SectionGridCarousel>
