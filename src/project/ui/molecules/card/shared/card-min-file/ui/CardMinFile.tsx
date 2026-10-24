// @ts-nocheck
import type { CardMinFileProps } from "../data/types";
import CardImage from "../../card-image/ui/CardImage";

export default function CardMinFile({ imageProps, active = false, onClick }: CardMinFileProps) {
    return (
        <button
            type="button"
            className={`relative shrink-0 size-14 overflow-hidden rounded-md border border-blender-border cursor-pointer transition-colors [&>img]:h-full [&>img]:w-full [&>img]:rounded-none [&>img]:object-cover [&>img]:transition-none ${active ? "border-blue-500 ring-1 ring-blue-500" : ""}`}
            onClick={onClick}
        >
            {imageProps && <CardImage {...imageProps} />}
        </button>
    );
}
