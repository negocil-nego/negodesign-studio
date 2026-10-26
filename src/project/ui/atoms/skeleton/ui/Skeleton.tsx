// @ts-nocheck
import type { SkeletonProps } from "../data/types";
import { StyleComponent } from "../../../../data/style.component";

export default function Skeleton({ className = "" }: SkeletonProps) {
    return (
        <div
            aria-hidden="true"
            className={`${StyleComponent.skeleton.default} ${className}`}
        />
    );
}
