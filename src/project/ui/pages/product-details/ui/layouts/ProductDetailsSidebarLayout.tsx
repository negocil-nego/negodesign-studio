// @ts-nocheck
import { StyleComponent } from "../../../../../data/style.component";
import ProductGallery from "../../shared/product-gallery/ui/ProductGallery";
import ProductInfo from "../../shared/product-info/ui/ProductInfo";
import ProductRating from "../../shared/product-rating/ui/ProductRating";
import ProductSimiliar from "../../shared/product-similiar/ui/ProductSimiliar";
import ProductTabs from "../../shared/product-tabs/ui/ProductTabs";
import type { ProductDetailsLayoutProps } from "../../data/types";

export default function ProductDetailsSidebarLayout({
  productGalleryProps, productInfoProps, productTabsProps, productRatingProps, productSimiliarProps,
}: ProductDetailsLayoutProps) {
  const S = StyleComponent.pages.productDetails;

  return (
    <>
      <section className={S.heroGrid}>
        <div className={S.galleryCol}>{productGalleryProps && <ProductGallery {...productGalleryProps} />}</div>
        <div className={S.infoCol}>{productInfoProps && <ProductInfo {...productInfoProps} />}</div>
      </section>

      <div className={S.sidebarLayout}>
        <div className={S.mainCol}>
          {productTabsProps && (
            <section className={S.sectionCard}>
              <ProductTabs {...productTabsProps} />
            </section>
          )}
          {productRatingProps && (
            <section className={S.sectionCard}>
              <ProductRating {...productRatingProps} />
            </section>
          )}
        </div>
        <div className={S.sideCol}>
          {productSimiliarProps && <ProductSimiliar {...productSimiliarProps} variant="sidebar" />}
        </div>
      </div>
    </>
  );
}
