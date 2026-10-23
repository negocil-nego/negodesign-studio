export type CardImageProps = {
    src: string;
    alt?: string;
    srcset?: string;
    sizes?: string;
    loading?: "lazy" | "eager";
    fetchpriority?: "high" | "low" | "auto";
}
