<script lang="ts">
  import CardProduct from "../../../../../../molecules/card/product/ui/CardProduct.svelte";
  import CardButton from "../../../../../../molecules/card/shared/card-button/ui/CardButton.svelte";
  import CardDescription from "../../../../../../molecules/card/shared/card-description/ui/CardDescription.svelte";
  import CardTitle from "../../../../../../molecules/card/shared/card-title/ui/CardTitle.svelte";
  import type { SimiliarGridProps } from "../../data/types";

  let {
    headerAlign = "left",
    titleProps,
    descriptionProps,
    columns = 4,
    items = [],
    moreButtonProps,
    className = "",
  }: SimiliarGridProps = $props();

  const gridColsClass = $derived(
    columns === 5
      ? "grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5"
      : columns === 3
        ? "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3"
        : "grid-cols-1 sm:grid-cols-2 lg:grid-cols-4",
  );
</script>

<section class={`space-y-6 ${className}`}>
  <div
    class={headerAlign === "center"
      ? "flex flex-col items-center text-center space-y-1.5"
      : "flex flex-wrap items-end justify-between gap-4"}
  >
    <div>
      {#if titleProps}
        <CardTitle
          {...titleProps}
          className={titleProps.className ?? "text-xl font-semibold tracking-tight text-zinc-100 sm:text-2xl"}
        />
      {/if}
      {#if descriptionProps}
        <CardDescription {...descriptionProps} />
      {/if}
    </div>

    {#if headerAlign === "center"}
      <span class="h-1 w-12 rounded-full bg-blue-500"></span>
    {:else if moreButtonProps}
      <CardButton variant="outline" {...moreButtonProps} />
    {/if}
  </div>

  <div class={`grid gap-4 ${gridColsClass}`}>
    {#each items as item, idx (idx)}
      <CardProduct {...item} variant={item.variant ?? "01"} />
    {/each}
  </div>

  {#if headerAlign === "center" && moreButtonProps}
    <div class="flex justify-center pt-2">
      <CardButton variant="outline" {...moreButtonProps} />
    </div>
  {/if}
</section>
