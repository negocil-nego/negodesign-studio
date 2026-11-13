<script setup lang="ts">
import { StyleComponent } from "../../../../../data/style.component";
import ProductGallery from "../../shared/product-gallery/ui/ProductGallery.vue";
import ProductInfo from "../../shared/product-info/ui/ProductInfo.vue";
import ProductRating from "../../shared/product-rating/ui/ProductRating.vue";
import ProductSimiliar from "../../shared/product-similiar/ui/ProductSimiliar.vue";
import ProductTabs from "../../shared/product-tabs/ui/ProductTabs.vue";
import type { ProductDetailsLayoutProps } from "../../data/types";

defineProps<ProductDetailsLayoutProps>();

const S = StyleComponent.pages.productDetails;
</script>

<template>
  <section :class="S.heroGrid">
    <div :class="S.galleryCol">
      <ProductGallery v-if="productGalleryProps" v-bind="productGalleryProps" layout="bottom-thumbs" />
    </div>
    <div :class="S.infoCol">
      <ProductInfo v-if="productInfoProps" v-bind="productInfoProps" />
    </div>
  </section>

  <div class="grid gap-8 lg:grid-cols-12 lg:items-start">
    <div class="lg:col-span-5">
      <div v-if="productGalleryProps" :class="S.sectionCard">
        <ProductGallery v-bind="productGalleryProps" layout="grid-gallery" />
      </div>
    </div>
    <div class="space-y-6 lg:col-span-7">
      <section v-if="productTabsProps" :class="S.sectionCard">
        <ProductTabs v-bind="productTabsProps" specsStyle="table" />
      </section>
      <section v-if="productRatingProps" :class="S.sectionCard">
        <ProductRating v-bind="productRatingProps" />
      </section>
    </div>
  </div>

  <section v-if="productSimiliarProps" :class="S.sectionCard">
    <ProductSimiliar v-bind="productSimiliarProps" variant="grid" />
  </section>
</template>
