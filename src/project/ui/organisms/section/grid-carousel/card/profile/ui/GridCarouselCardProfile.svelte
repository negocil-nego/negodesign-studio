<script lang="ts">
  import type { GridCarouselCardProfileProps } from "../data/types";
  import CardProfile from "../../../../../../molecules/card/profile/ui/CardProfile.svelte";
  import SectionGridCarousel from "../../../ui/SectionGridCarousel.svelte";

  let { items, sectionGridCarouselProps }: GridCarouselCardProfileProps = $props();
</script>

<SectionGridCarousel {...sectionGridCarouselProps}>
  {#each items as item}
    <CardProfile {...item} />
  {/each}
</SectionGridCarousel>
