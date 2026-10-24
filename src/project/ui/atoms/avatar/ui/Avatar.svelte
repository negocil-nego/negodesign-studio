<script lang="ts">
    import Skeleton from "../../skeleton/ui/Skeleton.svelte";
    import type { AvatarProps } from "../data/types";

    let { src, isLoading, onClick, className }: AvatarProps = $props();
</script>

{#if isLoading}
    <Skeleton className={className || "w-12 h-12 rounded-lg"} />
{:else if src}
    <button onclick={onClick} class={className}>
        <img {src} alt={src} class={className || "w-12 h-12 rounded-lg"} />
    </button>
{/if}
