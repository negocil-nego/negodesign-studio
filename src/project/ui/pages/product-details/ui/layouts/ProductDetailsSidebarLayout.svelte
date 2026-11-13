<script lang="ts">
  import { StyleComponent } from "../../../../../data/style.component";
  import ProductGallery from "../../shared/product-gallery/ui/ProductGallery.svelte";
  import ProductInfo from "../../shared/product-info/ui/ProductInfo.svelte";
  import ProductRating from "../../shared/product-rating/ui/ProductRating.svelte";
  import ProductSimiliar from "../../shared/product-similiar/ui/ProductSimiliar.svelte";
  import ProductTabs from "../../shared/product-tabs/ui/ProductTabs.svelte";
  import type { ProductDetailsLayoutProps } from "../../data/types";

  let {
    productGalleryProps, productInfoProps, productTabsProps,
    productRatingProps, productSimiliarProps,
  }: ProductDetailsLayoutProps = $props();

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

<div class={S.sidebarLayout}>
  <div class={S.mainCol}>
    {#if productTabsProps}
      <section class={S.sectionCard}><ProductTabs {...productTabsProps} /></section>
    {/if}
    {#if productRatingProps}
      <section class={S.sectionCard}><ProductRating {...productRatingProps} /></section>
    {/if}
  </div>
  <div class={S.sideCol}>
    {#if productSimiliarProps}<ProductSimiliar {...productSimiliarProps} variant="sidebar" />{/if}
  </div>
</div>
