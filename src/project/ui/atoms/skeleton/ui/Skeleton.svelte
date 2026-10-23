<script lang="ts">
    import type { SkeletonProps } from "../../data/types";
    let { className = "" }: SkeletonProps = $props();
</script>

<div
    aria-hidden="true"
    class={`relative overflow-hidden rounded-xl bg-gray-200
        before:absolute before:inset-0 before:-translate-x-full
        before:animate-shimmer
        before:bg-linear-to-r before:from-transparent before:via-white/70 before:to-transparent
        ${className}`}
></div>