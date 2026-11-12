import type { BreadcrumbProps } from "../../../molecules/breadcrumb/data/types";
import type { FooterProps } from "../../../organisms/footer/data/types";
import type { MenuProps } from "../../../organisms/menu/simple/data/types";
import type { GridCardMediaProps } from "../../../organisms/section/grid/card/media/data/types";
import type { ProductGalleryProps } from "../shared/product-gallery/data/types";
import type { ProductInfoProps } from "../shared/product-info/data/types";
import type { ProductRatingProps } from "../shared/product-rating/data/types";
import type { ProductSimiliarProps } from "../shared/product-similiar/data/types";
import type { ProductTabsProps } from "../shared/product-tabs/data/types";

export type {
  ProductGalleryProps,
  ProductInfoProps,
  ProductRatingProps,
  ProductSimiliarProps,
  ProductTabsProps,
};

export type PageProductDetailsVariant =
  | "sidebar"
  | "classic"
  | "minimal"
  | "showcase"
  | "catalog"
  | "fresh";

export interface PageProductDetailsProps {
  variant?: PageProductDetailsVariant;
  topBarText?: string;
  topBarRightText?: string;
  menuProps?: MenuProps;
  breadcrumbProps?: BreadcrumbProps;
  bannerTitle?: string;
  backButtonText?: string;
  productGalleryProps?: ProductGalleryProps;
  productInfoProps?: ProductInfoProps;
  productTabsProps?: ProductTabsProps;
  productRatingProps?: ProductRatingProps;
  productSimiliarProps?: ProductSimiliarProps;
  featureCardsProps?: GridCardMediaProps;
  footerProps?: FooterProps;
  className?: string;
}
