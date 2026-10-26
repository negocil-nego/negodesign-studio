// @ts-nocheck
import Skeleton from "../../skeleton/ui/Skeleton";
import { StyleComponent } from "../../../../data/style.component";
import type { AvatarProps } from "../data/types";

export default function Avatar({ src, isLoading, onClick, className }: AvatarProps) {
    if (isLoading) {
        return <Skeleton className={className || StyleComponent.avatar.default} />;
    }
    if (src) {
        return (
            <button onClick={onClick} className={className}>
                <img src={src} alt={src} className={className || StyleComponent.avatar.default} />
            </button>
        );
    }
    return null;
}
