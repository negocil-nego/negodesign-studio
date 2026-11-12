import type { CardButtonProps } from "../../../../../molecules/card/shared/card-button/data/types";
import type { CardDescriptionProps } from "../../../../../molecules/card/shared/card-description/data/types";
import type { CardImageProps } from "../../../../../molecules/card/shared/card-image/data/types";
import type { CardRatingProps } from "../../../../../molecules/card/shared/card-rating/data/types";
import type { CardTitleProps } from "../../../../../molecules/card/shared/card-title/data/types";

export interface ProductRatingBreakdownItem {
  stars: number;
  label?: string;
  percentage: number;
}

export interface ProductReviewItemProps {
  id?: string;
  author: string;
  verifiedText?: string;
  date?: string;
  avatarSrc?: string;
  ratingProps?: CardRatingProps;
  headline?: string;
  descriptionProps?: CardDescriptionProps;
  mediaImages?: CardImageProps[];
  likesCount?: number;
}

export interface ProductRatingProps {
  titleProps?: CardTitleProps;
  summaryLayout?: "bars-left" | "score-left";
  averageScore?: string;
  ratingProps?: CardRatingProps;
  totalReviewsText?: string;
  recommendedPercent?: string;
  recommendedLabel?: string;
  writeReviewButtonProps?: CardButtonProps;
  breakdown?: ProductRatingBreakdownItem[];
  filterTitle?: string;
  resultCountText?: string;
  reviews?: ProductReviewItemProps[];
  loadMoreText?: string;
  className?: string;
}
