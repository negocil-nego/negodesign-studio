// @ts-nocheck
import Skeleton from "../../skeleton/ui/Skeleton";
import { StyleComponent } from "../../../../data/style.component";
import type { ImageProps } from "../data/types";

export default function Image({ src, isLoading, onClick, className }: ImageProps) {
    if (isLoading) {
        return <Skeleton className={className || StyleComponent.image.default} />;
    }
    if (src) {
        return (
            <button onClick={onClick} className={className}>
                <img src={src} alt={src} className={className || StyleComponent.image.default} />
            </button>
        );
    }
    return null;
}
