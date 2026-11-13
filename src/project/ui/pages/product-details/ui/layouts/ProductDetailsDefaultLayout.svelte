<script lang="ts">
  import GridCardMedia from "../../../../organisms/section/grid/card/media/ui/GridCardMedia.svelte";
  import { StyleComponent } from "../../../../../data/style.component";
  import ProductGallery from "../../shared/product-gallery/ui/ProductGallery.svelte";
  import ProductInfo from "../../shared/product-info/ui/ProductInfo.svelte";
  import ProductRating from "../../shared/product-rating/ui/ProductRating.svelte";
  import ProductSimiliar from "../../shared/product-similiar/ui/ProductSimiliar.svelte";
  import ProductTabs from "../../shared/product-tabs/ui/ProductTabs.svelte";
  import type { ProductDetailsDefaultLayoutProps } from "../../data/types";

  let {
    variant, productGalleryProps, productInfoProps, productTabsProps,
    productRatingProps, productSimiliarProps, featureCardsProps,
  }: ProductDetailsDefaultLayoutProps = $props();

  const S = StyleComponent.pages.productDetails;
</script>

<section class={S.heroGrid}>
  <div class={S.galleryCol}>
    {#if productGalleryProps}<ProductGallery {...productGalleryProps} />{/if}
  </div>
  <div class={S.infoCol}>
    {#if productInfoProps}<ProductInfo {...productInfoProps} />{/if}
  </div>
</section>

{#if featureCardsProps}
  <section class={S.sectionCard}><GridCardMedia {...featureCardsProps} /></section>
{/if}

{#if productTabsProps && variant !== "minimal"}
  <section class={S.sectionCard}><ProductTabs {...productTabsProps} /></section>
{/if}

{#if productRatingProps && variant !== "minimal"}
  <section class={S.sectionCard}><ProductRating {...productRatingProps} /></section>
{/if}

{#if productSimiliarProps}
  <section class={S.sectionCard}><ProductSimiliar {...productSimiliarProps} /></section>
{/if}
