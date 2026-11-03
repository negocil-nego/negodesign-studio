<script lang="ts">
  import type { CarouselCardMediaProps } from "../data/types";
  import CardMedia from "../../../../../../molecules/card/media/ui/CardMedia.svelte";
  import SectionCarouselButtons from "../../../buttons/ui/SectionCarouselButtons.svelte";

  let { items, sectionCarouselButtonsProps }: CarouselCardMediaProps = $props();
</script>

<SectionCarouselButtons {...sectionCarouselButtonsProps}>
  {#each items as item}
    <CardMedia {...item} />
  {/each}
</SectionCarouselButtons>
