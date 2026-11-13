// @ts-nocheck
import GridCardMedia from "../../../../organisms/section/grid/card/media/ui/GridCardMedia";
import { StyleComponent } from "../../../../../data/style.component";
import ProductGallery from "../../shared/product-gallery/ui/ProductGallery";
import ProductInfo from "../../shared/product-info/ui/ProductInfo";
import ProductRating from "../../shared/product-rating/ui/ProductRating";
import ProductSimiliar from "../../shared/product-similiar/ui/ProductSimiliar";
import ProductTabs from "../../shared/product-tabs/ui/ProductTabs";
import type { ProductDetailsDefaultLayoutProps } from "../../data/types";

export default function ProductDetailsDefaultLayout({
  variant, productGalleryProps, productInfoProps, productTabsProps,
  productRatingProps, productSimiliarProps, featureCardsProps,
}: ProductDetailsDefaultLayoutProps) {
  const S = StyleComponent.pages.productDetails;

  return (
    <>
      <section className={S.heroGrid}>
        <div className={S.galleryCol}>{productGalleryProps && <ProductGallery {...productGalleryProps} />}</div>
        <div className={S.infoCol}>{productInfoProps && <ProductInfo {...productInfoProps} />}</div>
      </section>

      {featureCardsProps && (
        <section className={S.sectionCard}>
          <GridCardMedia {...featureCardsProps} />
        </section>
      )}

      {productTabsProps && variant !== "minimal" && (
        <section className={S.sectionCard}>
          <ProductTabs {...productTabsProps} />
        </section>
      )}

      {productRatingProps && variant !== "minimal" && (
        <section className={S.sectionCard}>
          <ProductRating {...productRatingProps} />
        </section>
      )}

      {productSimiliarProps && (
        <section className={S.sectionCard}>
          <ProductSimiliar {...productSimiliarProps} />
        </section>
      )}
    </>
  );
}
