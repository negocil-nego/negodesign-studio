<script lang="ts">
  import type { GridCardMediaProps } from "../data/types";
  import CardMedia from "../../../../../../molecules/card/media/ui/CardMedia.svelte";
  import SectionGrid from "../../../ui/SectionGrid.svelte";

  let { items, sectionGridProps }: GridCardMediaProps = $props();
</script>

<SectionGrid {...sectionGridProps}>
  {#each items as item}
    <CardMedia {...item} />
  {/each}
</SectionGrid>
