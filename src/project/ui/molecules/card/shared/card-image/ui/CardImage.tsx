// @ts-nocheck
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
            className="h-50 w-full rounded-md object-cover transition-opacity duration-500"
        />
    );
}
