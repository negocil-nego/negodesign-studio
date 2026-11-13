<script lang="ts">
  import CardProduct from "../../../../../../molecules/card/product/ui/CardProduct.svelte";
  import CardButton from "../../../../../../molecules/card/shared/card-button/ui/CardButton.svelte";
  import CardTitle from "../../../../../../molecules/card/shared/card-title/ui/CardTitle.svelte";
  import type { SimiliarSidebarProps } from "../../data/types";

  let { titleProps, items = [], moreButtonProps, className = "" }: SimiliarSidebarProps = $props();
</script>

<aside class={`space-y-4 rounded-2xl border border-blender-border bg-blender-card p-5 shadow-sm ${className}`}>
  {#if titleProps}
    <div class="border-b border-blender-border pb-3 text-center">
      <CardTitle
        {...titleProps}
        className={titleProps.className ?? "text-sm font-semibold text-zinc-100"}
      />
    </div>
  {/if}

  <div class="space-y-3">
    {#each items as item, idx (idx)}
      <CardProduct {...item} variant={item.variant ?? "02"} />
    {/each}
  </div>

  {#if moreButtonProps}
    <div class="pt-2 text-center">
      <CardButton variant="outline" {...moreButtonProps} />
    </div>
  {/if}
</aside>
