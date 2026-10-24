// @ts-nocheck
import Skeleton from "../../skeleton/ui/Skeleton";
import type { AvatarProps } from "../data/types";

export default function Avatar({ src, isLoading, onClick, className }: AvatarProps) {
    if (isLoading) {
        return <Skeleton className={className || "w-12 h-12 rounded-lg"} />;
    }
    if (src) {
        return (
            <button onClick={onClick} className={className}>
                <img src={src} alt={src} className={className || "w-12 h-12 rounded-lg"} />
            </button>
        );
    }
    return null;
}
