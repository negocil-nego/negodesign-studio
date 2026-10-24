// @ts-nocheck
import type { SkeletonProps } from "../data/types";

export default function Skeleton({ className = "" }: SkeletonProps) {
    return (
        <div
            aria-hidden="true"
            className={`relative overflow-hidden rounded-xl bg-gray-200 before:absolute before:inset-0 before:-translate-x-full before:animate-shimmer before:bg-linear-to-r before:from-transparent before:via-white/70 before:to-transparent ${className}`}
        />
    );
}
