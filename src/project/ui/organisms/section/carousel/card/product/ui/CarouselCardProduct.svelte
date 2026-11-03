<script lang="ts">
  import type { CarouselCardProductProps } from "../data/types";
  import CardProduct from "../../../../../../molecules/card/product/ui/CardProduct.svelte";
  import SectionCarouselButtons from "../../../buttons/ui/SectionCarouselButtons.svelte";

  let { items, sectionCarouselButtonsProps }: CarouselCardProductProps = $props();
</script>

<SectionCarouselButtons {...sectionCarouselButtonsProps}>
  {#each items as item}
    <CardProduct {...item} />
  {/each}
</SectionCarouselButtons>
