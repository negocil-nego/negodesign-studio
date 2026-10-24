<script lang="ts">
    import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
    import { StyleComponent } from "../../../../../../data/style.component";
    import type { CardTagsProps } from "../data/types";

    let { tags, className, itemClassName }: CardTagsProps = $props();
</script>

<div class={className ?? StyleComponent.card.tags.root}>
    {#each tags as tag}
        <div class={itemClassName ?? StyleComponent.card.tags.item}>
            {#if tag.icon}
                <Icon name={tag.icon} className={StyleComponent.card.tags.icon} />
            {/if}
            <span class={StyleComponent.card.tags.value}>{tag.title}</span>
            {#if tag.description}
                <span class={StyleComponent.card.tags.label}>{tag.description}</span>
            {/if}
        </div>
    {/each}
</div>
