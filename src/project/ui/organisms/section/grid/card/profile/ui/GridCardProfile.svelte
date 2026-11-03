<script lang="ts">
  import type { GridCardProfileProps } from "../data/types";
  import CardProfile from "../../../../../../molecules/card/profile/ui/CardProfile.svelte";
  import SectionGrid from "../../../ui/SectionGrid.svelte";

  let { items, sectionGridProps }: GridCardProfileProps = $props();
</script>

<SectionGrid {...sectionGridProps}>
  {#each items as item}
    <CardProfile {...item} />
  {/each}
</SectionGrid>
