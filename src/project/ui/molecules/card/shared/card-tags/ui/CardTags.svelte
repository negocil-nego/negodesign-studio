<script lang="ts">
    import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
    import type { CardTagsProps } from "../data/types";

    let { tags, className, itemClassName }: CardTagsProps = $props();
</script>

<div class={className ?? "flex gap-4 w-full border-y border-gray-100 py-4"}>
    {#each tags as tag}
        <div class={itemClassName ?? "flex flex-col items-center justify-center flex-1 first:border-l-0 border-l border-gray-100"}>
            {#if tag.icon}
                <Icon name={tag.icon} className="mb-1" />
            {/if}
            <span class="text-lg font-bold text-gray-900">{tag.title}</span>
            {#if tag.description}
                <span class="text-sm text-gray-500">{tag.description}</span>
            {/if}
        </div>
    {/each}
</div>
