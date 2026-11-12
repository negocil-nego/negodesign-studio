<script setup lang="ts">
import Icon from "../../../atoms/icon/ui/Icon.vue";
import Breadcrumbs from "../../../molecules/breadcrumb/ui/Breadcrumbs.vue";
import Footer from "../../../organisms/footer/ui/Footer.vue";
import Menu from "../../../organisms/menu/simple/ui/Menu.vue";
import GridCardMedia from "../../../organisms/section/grid/card/media/ui/GridCardMedia.vue";
import { StyleComponent } from "../../../../data/style.component";
import type { PageProductDetailsProps } from "../data/types";
import ProductGallery from "../shared/product-gallery/ui/ProductGallery.vue";
import ProductInfo from "../shared/product-info/ui/ProductInfo.vue";
import ProductRating from "../shared/product-rating/ui/ProductRating.vue";
import ProductSimiliar from "../shared/product-similiar/ui/ProductSimiliar.vue";
import ProductTabs from "../shared/product-tabs/ui/ProductTabs.vue";

withDefaults(defineProps<PageProductDetailsProps>(), {
  variant: "sidebar",
  className: "",
});

const S = StyleComponent.pages.productDetails;
</script>

<template>
  <div :class="`${S.root} ${className}`">
    <div v-if="topBarText || topBarRightText" :class="S.topUtilityBar">
      <div :class="S.topUtilityInner">
        <span>{{ topBarText ?? "" }}</span>
        <span v-if="topBarRightText">{{ topBarRightText }}</span>
      </div>
    </div>

    <header v-if="menuProps" :class="S.navWrap">
      <div :class="S.navInner">
        <Menu v-bind="menuProps" />
      </div>
    </header>

    <div v-if="bannerTitle" :class="S.bannerHeader">
      <h1 :class="S.bannerTitle">{{ bannerTitle }}</h1>
      <div v-if="breadcrumbProps" class="mt-2 flex justify-center">
        <Breadcrumbs v-bind="breadcrumbProps" />
      </div>
    </div>
    <div v-else-if="breadcrumbProps || backButtonText" :class="S.breadcrumbBar">
      <div class="flex flex-wrap items-center gap-3">
        <button v-if="backButtonText" type="button" :class="S.backBtn">
          <Icon name="ArrowLeft01" className="size-3.5" />
          <span>{{ backButtonText }}</span>
        </button>
        <Breadcrumbs v-if="breadcrumbProps" v-bind="breadcrumbProps" />
      </div>
    </div>

    <main :class="S.container">
      <template v-if="variant === 'sidebar'">
        <section :class="S.heroGrid">
          <div :class="S.galleryCol">
            <ProductGallery v-if="productGalleryProps" v-bind="productGalleryProps" />
          </div>
          <div :class="S.infoCol">
            <ProductInfo v-if="productInfoProps" v-bind="productInfoProps" />
          </div>
        </section>

        <div :class="S.sidebarLayout">
          <div :class="S.mainCol">
            <section v-if="productTabsProps" :class="S.sectionCard">
              <ProductTabs v-bind="productTabsProps" />
            </section>
            <section v-if="productRatingProps" :class="S.sectionCard">
              <ProductRating v-bind="productRatingProps" />
            </section>
          </div>
          <div :class="S.sideCol">
            <ProductSimiliar v-if="productSimiliarProps" v-bind="productSimiliarProps" variant="sidebar" />
          </div>
        </div>
      </template>

      <template v-else>
        <section :class="S.heroGrid">
          <div :class="S.galleryCol">
            <ProductGallery v-if="productGalleryProps" v-bind="productGalleryProps" />
          </div>
          <div :class="S.infoCol">
            <ProductInfo v-if="productInfoProps" v-bind="productInfoProps" />
          </div>
        </section>

        <section v-if="featureCardsProps" :class="S.sectionCard">
          <GridCardMedia v-bind="featureCardsProps" />
        </section>

        <section v-if="productTabsProps && variant !== 'minimal'" :class="S.sectionCard">
          <ProductTabs v-bind="productTabsProps" />
        </section>

        <section v-if="productRatingProps && variant !== 'minimal'" :class="S.sectionCard">
          <ProductRating v-bind="productRatingProps" />
        </section>

        <section v-if="productSimiliarProps" :class="S.sectionCard">
          <ProductSimiliar v-bind="productSimiliarProps" />
        </section>
      </template>
    </main>

    <Footer v-if="footerProps" v-bind="footerProps" />
  </div>
</template>
