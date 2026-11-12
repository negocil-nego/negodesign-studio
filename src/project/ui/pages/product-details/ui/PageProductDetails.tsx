// @ts-nocheck
import Icon from "../../../atoms/icon/ui/Icon";
import Breadcrumbs from "../../../molecules/breadcrumb/ui/Breadcrumbs";
import Footer from "../../../organisms/footer/ui/Footer";
import Menu from "../../../organisms/menu/simple/ui/Menu";
import GridCardMedia from "../../../organisms/section/grid/card/media/ui/GridCardMedia";
import { StyleComponent } from "../../../../data/style.component";
import type { PageProductDetailsProps } from "../data/types";
import ProductGallery from "../shared/product-gallery/ui/ProductGallery";
import ProductInfo from "../shared/product-info/ui/ProductInfo";
import ProductRating from "../shared/product-rating/ui/ProductRating";
import ProductSimiliar from "../shared/product-similiar/ui/ProductSimiliar";
import ProductTabs from "../shared/product-tabs/ui/ProductTabs";

export default function PageProductDetails({
  variant = "sidebar",
  topBarText,
  topBarRightText,
  menuProps,
  breadcrumbProps,
  bannerTitle,
  backButtonText,
  productGalleryProps,
  productInfoProps,
  productTabsProps,
  productRatingProps,
  productSimiliarProps,
  featureCardsProps,
  footerProps,
  className = "",
}: PageProductDetailsProps) {
  const S = StyleComponent.pages.productDetails;

  return (
    <div className={`${S.root} ${className}`}>
      {(topBarText || topBarRightText) && (
        <div className={S.topUtilityBar}>
          <div className={S.topUtilityInner}>
            <span>{topBarText ?? ""}</span>
            {topBarRightText && <span>{topBarRightText}</span>}
          </div>
        </div>
      )}

      {menuProps && (
        <header className={S.navWrap}>
          <div className={S.navInner}>
            <Menu {...menuProps} />
          </div>
        </header>
      )}

      {bannerTitle ? (
        <div className={S.bannerHeader}>
          <h1 className={S.bannerTitle}>{bannerTitle}</h1>
          {breadcrumbProps && (
            <div className="mt-2 flex justify-center">
              <Breadcrumbs {...breadcrumbProps} />
            </div>
          )}
        </div>
      ) : (
        (breadcrumbProps || backButtonText) && (
          <div className={S.breadcrumbBar}>
            <div className="flex flex-wrap items-center gap-3">
              {backButtonText && (
                <button type="button" className={S.backBtn}>
                  <Icon name="ArrowLeft01" className="size-3.5" />
                  <span>{backButtonText}</span>
                </button>
              )}
              {breadcrumbProps && <Breadcrumbs {...breadcrumbProps} />}
            </div>
          </div>
        )
      )}

      <main className={S.container}>
        {variant === "sidebar" ? (
          <>
            <section className={S.heroGrid}>
              <div className={S.galleryCol}>
                {productGalleryProps && <ProductGallery {...productGalleryProps} />}
              </div>
              <div className={S.infoCol}>
                {productInfoProps && <ProductInfo {...productInfoProps} />}
              </div>
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
        ) : (
          <>
            <section className={S.heroGrid}>
              <div className={S.galleryCol}>
                {productGalleryProps && <ProductGallery {...productGalleryProps} />}
              </div>
              <div className={S.infoCol}>
                {productInfoProps && <ProductInfo {...productInfoProps} />}
              </div>
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
        )}
      </main>

      {footerProps && <Footer {...footerProps} />}
    </div>
  );
}
