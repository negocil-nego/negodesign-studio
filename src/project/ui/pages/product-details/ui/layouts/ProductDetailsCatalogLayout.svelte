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
    {#if productGalleryProps}<ProductGallery {...productGalleryProps} layout="bottom-thumbs" />{/if}
  </div>
  <div class={S.infoCol}>
    {#if productInfoProps}<ProductInfo {...productInfoProps} />{/if}
  </div>
</section>

<div class="grid gap-8 lg:grid-cols-12 lg:items-start">
  <div class="lg:col-span-5">
    {#if productGalleryProps}
      <div class={S.sectionCard}><ProductGallery {...productGalleryProps} layout="grid-gallery" /></div>
    {/if}
  </div>
  <div class="space-y-6 lg:col-span-7">
    {#if productTabsProps}
      <section class={S.sectionCard}><ProductTabs {...productTabsProps} specsStyle="table" /></section>
    {/if}
    {#if productRatingProps}
      <section class={S.sectionCard}><ProductRating {...productRatingProps} /></section>
    {/if}
  </div>
</div>

{#if productSimiliarProps}
  <section class={S.sectionCard}><ProductSimiliar {...productSimiliarProps} variant="grid" /></section>
{/if}
