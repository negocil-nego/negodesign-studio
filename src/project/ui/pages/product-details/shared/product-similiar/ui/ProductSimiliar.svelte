<script lang="ts">
  import type { ProductSimiliarProps } from "../data/types";
  import SimiliarCarousel from "./similiar/SimiliarCarousel.svelte";
  import SimiliarGrid from "./similiar/SimiliarGrid.svelte";
  import SimiliarSidebar from "./similiar/SimiliarSidebar.svelte";

  let {
    variant = "grid",
    headerAlign = "left",
    titleProps = { title: "Similar Products" },
    descriptionProps,
    columns = 4,
    items = [],
    moreButtonProps,
    className = "",
  }: ProductSimiliarProps = $props();
</script>

{#if variant === "sidebar"}
  <SimiliarSidebar {titleProps} {items} {moreButtonProps} {className} />
{:else if variant === "carousel"}
  <SimiliarCarousel {titleProps} {descriptionProps} {items} {moreButtonProps} {className} />
{:else}
  <SimiliarGrid {headerAlign} {titleProps} {descriptionProps} {columns} {items} {moreButtonProps} {className} />
{/if}
