// @ts-nocheck
import { StyleComponent } from "../../../../../../data/style.component";
import type { CardImageProps } from "../data/types";

export default function CardImage({ src, alt, srcset, sizes, loading = "lazy", fetchpriority }: CardImageProps) {
    return (
        <img
            src={src}
            srcSet={srcset}
            sizes={sizes}
            alt={alt}
            fetchPriority={fetchpriority}
            loading={loading}
            decoding="async"
            className={`${StyleComponent.card.media.default} transition-opacity duration-500`}
        />
    );
}
