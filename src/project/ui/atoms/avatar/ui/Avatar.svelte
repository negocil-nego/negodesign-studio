<script lang="ts">
    import Skeleton from "../../skeleton/ui/Skeleton.svelte";
    import { StyleComponent } from "../../../../data/style.component";
    import type { AvatarProps } from "../data/types";

    let { src, isLoading, onClick, className }: AvatarProps = $props();
</script>

{#if isLoading}
    <Skeleton className={className || StyleComponent.avatar.default} />
{:else if src}
    <button onclick={onClick} class={className}>
        <img {src} alt={src} class={className || StyleComponent.avatar.default} />
    </button>
{/if}
