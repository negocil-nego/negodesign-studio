<script lang="ts">
  import type { GridCardProductProps } from "../data/types";
  import CardProduct from "../../../../../../molecules/card/product/ui/CardProduct.svelte";
  import SectionGrid from "../../../ui/SectionGrid.svelte";

  let { items, sectionGridProps }: GridCardProductProps = $props();
</script>

<SectionGrid {...sectionGridProps}>
  {#each items as item}
    <CardProduct {...item} />
  {/each}
</SectionGrid>
