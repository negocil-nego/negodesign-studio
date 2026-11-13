import type { CardButtonProps } from "../../../../../molecules/card/shared/card-button/data/types";
import type { CardCartButtonProps } from "../../../../../molecules/card/shared/card-cart-button/data/types";
import type { CardDescriptionProps } from "../../../../../molecules/card/shared/card-description/data/types";
import type { CardFavoriteButtonProps } from "../../../../../molecules/card/shared/card-favorite-button/data/types";
import type { CardPriceProps } from "../../../../../molecules/card/shared/card-price/data/types";
import type { CardRatingProps } from "../../../../../molecules/card/shared/card-rating/data/types";
import type { CardTagsProps } from "../../../../../molecules/card/shared/card-tags/data/types";
import type { CardTitleProps } from "../../../../../molecules/card/shared/card-title/data/types";
import type { ProductTabsProps } from "../../product-tabs/data/types";
import type {
  ProductAccordionItem,
  ProductColorOption,
  ProductMetaItem,
  ProductPurchaseOption,
  ProductSizeOption,
  ProductTrustBadge,
} from "./types";

export interface ProductInfoHeaderProps {
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
  calloutText?: string;
}

export interface ProductInfoOptionsProps {
  colorsLabel?: string;
  colors?: ProductColorOption[];
  sizesLabel?: string;
  sizes?: ProductSizeOption[];
  purchaseOptions?: ProductPurchaseOption[];
  showQuantity?: boolean;
  quantity?: number;
  selectedColor?: string;
  selectedSize?: string;
  selectedPurchase?: string;
  stockCountText?: string;
  subtotalText?: string;
  deliveryText?: string;
  urgencyText?: string;
  onQuantityChange?: (value: number) => void;
  onSelectedColorChange?: (value: string) => void;
  onSelectedSizeChange?: (value: string) => void;
  onSelectedPurchaseChange?: (value: string) => void;
}

export interface ProductInfoActionsProps {
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
}
