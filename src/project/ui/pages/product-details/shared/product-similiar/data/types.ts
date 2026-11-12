import type { CardProductProps } from "../../../../../molecules/card/product/data/types";
import type { CardButtonProps } from "../../../../../molecules/card/shared/card-button/data/types";
import type { CardDescriptionProps } from "../../../../../molecules/card/shared/card-description/data/types";
import type { CardTitleProps } from "../../../../../molecules/card/shared/card-title/data/types";

export interface ProductSimiliarProps {
  variant?: "grid" | "sidebar" | "carousel";
  headerAlign?: "left" | "center";
  titleProps?: CardTitleProps;
  descriptionProps?: CardDescriptionProps;
  columns?: 3 | 4 | 5;
  items: CardProductProps[];
  moreButtonProps?: CardButtonProps;
  className?: string;
}

export type ProductSimilarProps = ProductSimiliarProps;
