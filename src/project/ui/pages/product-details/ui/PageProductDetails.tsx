// @ts-nocheck
import Icon from "../../../atoms/icon/ui/Icon";
import Breadcrumbs from "../../../molecules/breadcrumb/ui/Breadcrumbs";
import Footer from "../../../organisms/footer/ui/Footer";
import Menu from "../../../organisms/menu/simple/ui/Menu";
import { StyleComponent } from "../../../../data/style.component";
import type { PageProductDetailsProps } from "../data/types";
import ProductDetailsSidebarLayout from "./layouts/ProductDetailsSidebarLayout";
import ProductDetailsDefaultLayout from "./layouts/ProductDetailsDefaultLayout";

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
  const sectionProps = {
    productGalleryProps,
    productInfoProps,
    productTabsProps,
    productRatingProps,
    productSimiliarProps,
  };

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
          <ProductDetailsSidebarLayout {...sectionProps} />
        ) : (
          <ProductDetailsDefaultLayout {...sectionProps} variant={variant} featureCardsProps={featureCardsProps} />
        )}
      </main>

      {footerProps && <Footer {...footerProps} />}
    </div>
  );
}
