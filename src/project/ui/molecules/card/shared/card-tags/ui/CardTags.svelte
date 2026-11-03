<script lang="ts">
  import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
  import { StyleComponent } from "../../../../../../data/style.component";
  import type { CardTagsProps } from "../data/types";

  let {
    tags,
    variant = "stats",
    className,
    itemClassName,
    titleClassName,
    descriptionClassName,
  }: CardTagsProps = $props();
  const tagStyle = $derived(
    variant === "buttons" ? StyleComponent.card.tags.buttons : StyleComponent.card.tags,
  );
</script>

<div class={className ?? tagStyle.root}>
  {#each tags as tag}
    <div class={itemClassName ?? tagStyle.item}>
      {#if tag.icon}
        <Icon name={tag.icon} className={tagStyle.icon} />
      {/if}
      <span class={titleClassName ?? tagStyle.value}>{tag.title}</span>
      {#if tag.description}
        <span class={descriptionClassName ?? tagStyle.label}>{tag.description}</span>
      {/if}
    </div>
  {/each}
</div>
