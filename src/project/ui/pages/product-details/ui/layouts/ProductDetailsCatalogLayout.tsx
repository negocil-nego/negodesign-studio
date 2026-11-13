// @ts-nocheck
import { StyleComponent } from "../../../../../data/style.component";
import ProductGallery from "../../shared/product-gallery/ui/ProductGallery";
import ProductInfo from "../../shared/product-info/ui/ProductInfo";
import ProductRating from "../../shared/product-rating/ui/ProductRating";
import ProductSimiliar from "../../shared/product-similiar/ui/ProductSimiliar";
import ProductTabs from "../../shared/product-tabs/ui/ProductTabs";
import type { ProductDetailsLayoutProps } from "../../data/types";

export default function ProductDetailsCatalogLayout({
  productGalleryProps, productInfoProps, productTabsProps, productRatingProps, productSimiliarProps,
}: ProductDetailsLayoutProps) {
  const S = StyleComponent.pages.productDetails;

  return (
    <>
      <section className={S.heroGrid}>
        <div className={S.galleryCol}>
          {productGalleryProps && <ProductGallery {...productGalleryProps} layout="bottom-thumbs" />}
        </div>
        <div className={S.infoCol}>{productInfoProps && <ProductInfo {...productInfoProps} />}</div>
      </section>

      <div className="grid gap-8 lg:grid-cols-12 lg:items-start">
        <div className="lg:col-span-5">
          {productGalleryProps && (
            <div className={S.sectionCard}>
              <ProductGallery {...productGalleryProps} layout="grid-gallery" />
            </div>
          )}
        </div>
        <div className="space-y-6 lg:col-span-7">
          {productTabsProps && (
            <section className={S.sectionCard}>
              <ProductTabs {...productTabsProps} specsStyle="table" />
            </section>
          )}
          {productRatingProps && (
            <section className={S.sectionCard}>
              <ProductRating {...productRatingProps} />
            </section>
          )}
        </div>
      </div>

      {productSimiliarProps && (
        <section className={S.sectionCard}>
          <ProductSimiliar {...productSimiliarProps} variant="grid" />
        </section>
      )}
    </>
  );
}
