<script lang="ts">
  import type { CarouselCardProfileProps } from "../data/types";
  import CardProfile from "../../../../../../molecules/card/profile/ui/CardProfile.svelte";
  import SectionCarouselButtons from "../../../buttons/ui/SectionCarouselButtons.svelte";

  let { items, sectionCarouselButtonsProps }: CarouselCardProfileProps = $props();
</script>

<SectionCarouselButtons {...sectionCarouselButtonsProps}>
  {#each items as item}
    <CardProfile {...item} />
  {/each}
</SectionCarouselButtons>
