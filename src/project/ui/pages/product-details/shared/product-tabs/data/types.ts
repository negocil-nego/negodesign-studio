import type { CardDescriptionProps } from "../../../../../molecules/card/shared/card-description/data/types";
import type { CardTitleProps } from "../../../../../molecules/card/shared/card-title/data/types";
import type { UnderlineNavProps } from "../../../../../molecules/underline-nav/data/types";

export interface ProductSpecItem {
  label: string;
  value: string;
}

export interface ProductTabContentItem {
  value: string;
  titleProps?: CardTitleProps;
  descriptionProps?: CardDescriptionProps;
  paragraphs?: string[];
  specs?: ProductSpecItem[];
  highlights?: string[];
}

export interface ProductTabsProps {
  underlineNavProps?: UnderlineNavProps;
  titleProps?: CardTitleProps;
  descriptionProps?: CardDescriptionProps;
  paragraphs?: string[];
  specs?: ProductSpecItem[];
  highlights?: string[];
  tabContents?: ProductTabContentItem[];
  specsStyle?: "inline" | "table";
  className?: string;
}
