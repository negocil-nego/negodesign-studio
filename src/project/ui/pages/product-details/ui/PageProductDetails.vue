<script setup lang="ts">
import { computed } from "vue";
import Icon from "../../../atoms/icon/ui/Icon.vue";
import Breadcrumbs from "../../../molecules/breadcrumb/ui/Breadcrumbs.vue";
import Footer from "../../../organisms/footer/ui/Footer.vue";
import Menu from "../../../organisms/menu/simple/ui/Menu.vue";
import { StyleComponent } from "../../../../data/style.component";
import type { PageProductDetailsProps } from "../data/types";
import ProductDetailsSidebarLayout from "./layouts/ProductDetailsSidebarLayout.vue";
import ProductDetailsDefaultLayout from "./layouts/ProductDetailsDefaultLayout.vue";

const props = withDefaults(defineProps<PageProductDetailsProps>(), {
  variant: "sidebar",
  className: "",
});

const S = StyleComponent.pages.productDetails;
const sectionProps = computed(() => ({
  productGalleryProps: props.productGalleryProps,
  productInfoProps: props.productInfoProps,
  productTabsProps: props.productTabsProps,
  productRatingProps: props.productRatingProps,
  productSimiliarProps: props.productSimiliarProps,
}));
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
      <ProductDetailsSidebarLayout v-if="variant === 'sidebar'" v-bind="sectionProps" />
      <ProductDetailsDefaultLayout v-else v-bind="sectionProps" :variant="variant" :feature-cards-props="featureCardsProps" />
    </main>

    <Footer v-if="footerProps" v-bind="footerProps" />
  </div>
</template>
