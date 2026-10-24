import type { CardButtonProps } from "../../shared/card-button/data/types";
import type { CardCartButtonProps } from "../../shared/card-cart-button/data/types";
import type { CardDescriptionProps } from "../../shared/card-description/data/types";
import type { CardFavoriteButtonProps } from "../../shared/card-favorite-button/data/types";
import type { CardImageProps } from "../../shared/card-image/data/types";
import type { CardLogoProps } from "../../shared/card-logo/data/types";
import type { CardRatingProps } from "../../shared/card-rating/data/types";
import type { CardThumbnailVideoProps } from "../../shared/card-thumbnail-video/data/types";
import type { CardTitleProps } from "../../shared/card-title/data/types";
import type { CardVideoProps } from "../../shared/card-video/data/types";
import type { CardTagsProps } from "../../shared/card-tags/data/types";
import type { CardActionsButtonProps } from "../../shared/card-actions-button/data/types";

export type CardProfileBaseProps = {
    logoProps?: CardLogoProps;
    titleProps?: CardTitleProps;
    videoProps?: CardVideoProps;
    imageProps?: CardImageProps;
    ratingProps?: CardRatingProps;
    buttonProps?: CardButtonProps;
    cartButtonProps?: CardCartButtonProps;
    descriptionProps?: CardDescriptionProps;
    favoriteButtonProps?: CardFavoriteButtonProps;
    thumbnailVideoProps?: CardThumbnailVideoProps;
    tagsProps?: CardTagsProps;
    actionsProps?: CardActionsButtonProps;
}

export type CardProfileProps = CardProfileBaseProps & {
    variant?: "01" | "02";
}