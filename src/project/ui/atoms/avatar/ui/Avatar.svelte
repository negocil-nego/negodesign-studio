<script lang="ts">
    import Skeleton from "../../skeleton/ui/Skeleton.svelte";
    import type { AvatarProps } from "../data/types";

    let { src, isLoading, onClick }: AvatarProps = $props();
</script>

{#if isLoading}
    <Skeleton className="w-12 h-12 rounded-lg" />
{:else if src}
    <button onclick={onClick}>
        <img {src} alt={src} class="w-12 h-12 rounded-lg" />
    </button>
{/if}
