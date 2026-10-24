import type { CardImageProps } from "../../card-image/data/types";
import type { CardVideoProps } from "../../card-video/data/types";

export type CardThumbnailVideoProps = {
    videoProps: CardVideoProps;
    imageProps: CardImageProps;
    visibleButtonPlay?: boolean;
}