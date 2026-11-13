import type { CardButtonProps } from "../../../../../molecules/card/shared/card-button/data/types";
import type { CardCartButtonProps } from "../../../../../molecules/card/shared/card-cart-button/data/types";
import type { CardDescriptionProps } from "../../../../../molecules/card/shared/card-description/data/types";
import type { CardFavoriteButtonProps } from "../../../../../molecules/card/shared/card-favorite-button/data/types";
import type { CardPriceProps } from "../../../../../molecules/card/shared/card-price/data/types";
import type { CardRatingProps } from "../../../../../molecules/card/shared/card-rating/data/types";
import type { CardTagsProps } from "../../../../../molecules/card/shared/card-tags/data/types";
import type { CardTitleProps } from "../../../../../molecules/card/shared/card-title/data/types";
import type { ProductTabsProps } from "../../product-tabs/data/types";

export interface ProductColorOption {
  label: string;
  value: string;
  hex: string;
}

export interface ProductSizeOption {
  label: string;
  value: string;
  priceLabel?: string;
}

export interface ProductPurchaseOption {
  id: string;
  label: string;
  price: string;
}

export interface ProductTrustBadge {
  icon?: string;
  text: string;
}

export interface ProductMetaItem {
  label: string;
  value: string;
}

export interface ProductAccordionItem {
  title: string;
  count?: string;
  content: string;
  open?: boolean;
}

export type {
  ProductInfoActionsProps,
  ProductInfoHeaderProps,
  ProductInfoOptionsProps,
} from "./section-types";

export interface ProductInfoProps {
  category?: string;
  badge?: string;
  stockBadge?: string;
  titleProps?: CardTitleProps;
  subtitle?: string;
  ratingProps?: CardRatingProps;
  reviewCountText?: string;
  priceProps?: CardPriceProps;
  priceUnit?: string;
  oldPrice?: string;
  discountText?: string;
  socialProofText?: string;
  descriptionProps?: CardDescriptionProps;
  featuresTitle?: string;
  features?: string[];
  colorsLabel?: string;
  colors?: ProductColorOption[];
  sizesLabel?: string;
  sizes?: ProductSizeOption[];
  purchaseOptions?: ProductPurchaseOption[];
  showQuantity?: boolean;
  initialQuantity?: number;
  stockCountText?: string;
  subtotalText?: string;
  deliveryText?: string;
  urgencyText?: string;
  calloutText?: string;
  buttonProps?: CardButtonProps;
  secondaryButtonProps?: CardButtonProps;
  cartButtonProps?: CardCartButtonProps;
  favoriteButtonProps?: CardFavoriteButtonProps;
  wishlistText?: string;
  compareText?: string;
  tagsProps?: CardTagsProps;
  trustBadges?: ProductTrustBadge[];
  metaItems?: ProductMetaItem[];
  accordionItems?: ProductAccordionItem[];
  jumpLinks?: string[];
  inlineTabsProps?: ProductTabsProps;
  className?: string;
}
