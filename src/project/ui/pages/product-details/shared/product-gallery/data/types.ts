import type { CardImageProps } from "../../../../../molecules/card/shared/card-image/data/types";
import type { CardMinFileProps } from "../../../../../molecules/card/shared/card-min-file/data/types";
import type { CardThumbnailVideoProps } from "../../../../../molecules/card/shared/card-thumbnail-video/data/types";
import type { CardVideoProps } from "../../../../../molecules/card/shared/card-video/data/types";

export interface ProductGalleryProps {
  layout?: "bottom-thumbs" | "left-thumbs" | "grid-gallery";
  badge?: string;
  showArrows?: boolean;
  imageProps?: CardImageProps;
  videoProps?: CardVideoProps;
  thumbnailVideoProps?: CardThumbnailVideoProps;
  minFileProps?: CardMinFileProps[];
  galleryImages?: CardImageProps[];
  className?: string;
}
