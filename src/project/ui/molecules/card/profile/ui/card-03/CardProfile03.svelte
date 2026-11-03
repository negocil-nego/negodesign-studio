<script lang="ts">
  import { StyleComponent } from "../../../../../../data/style.component";
  import CardActionsButton from "../../../shared/card-actions-button/ui/CardActionsButton.svelte";
  import CardDescription from "../../../shared/card-description/ui/CardDescription.svelte";
  import CardImage from "../../../shared/card-image/ui/CardImage.svelte";
  import CardLogo from "../../../shared/card-logo/ui/CardLogo.svelte";
  import CardTags from "../../../shared/card-tags/ui/CardTags.svelte";
  import CardTitle from "../../../shared/card-title/ui/CardTitle.svelte";
  import CardThumbnailVideo from "../../../shared/card-thumbnail-video/ui/CardThumbnailVideo.svelte";
  import CardVideo from "../../../shared/card-video/ui/CardVideo.svelte";
  import type { CardProfileBaseProps } from "../../data/types";

  let {
    logoProps,
    titleProps,
    handleProps,
    videoProps,
    imageProps,
    thumbnailVideoProps,
    descriptionProps,
    tagsProps,
    actionsProps,
    editActionProps,
  }: CardProfileBaseProps = $props();

  const style = StyleComponent.card.profile.v03;
</script>

<aside class={style.root}>
  <!-- Cover -->
  <div class={style.cover}>
    {#if thumbnailVideoProps}
      <CardThumbnailVideo {...thumbnailVideoProps} />
    {:else if videoProps}
      <CardVideo {...videoProps} controls={false} />
    {:else if imageProps}
      <CardImage {...imageProps} className={style.coverMedia} />
    {:else}
      <div class={style.coverFallback}></div>
    {/if}

    <!-- Edit Profile action (top-right of cover) -->
    {#if editActionProps}
      <div class={style.editAction}>
        <CardActionsButton actions={editActionProps.actions} variant="icon-outline" />
      </div>
    {/if}

    <!-- Avatar anchored to bottom of cover -->
    {#if logoProps}
      <div class={style.logoWrapper}>
        <CardLogo {...logoProps} className={style.logoImage} />
      </div>
    {/if}
  </div>

  <!-- Body -->
  <div class={style.body}>
    <!-- Name row + Follow / Message buttons -->
    <div class={style.headRow}>
      <div class={style.nameGroup}>
        {#if titleProps}
          <CardTitle {...titleProps} className={style.name} />
        {/if}
        {#if handleProps}
          <span class={style.handle}>{handleProps}</span>
        {/if}
      </div>

      {#if actionsProps}
        <div class={style.actionsGroup}>
          <CardActionsButton actions={actionsProps.actions} variant="mixed" />
        </div>
      {/if}
    </div>

    <!-- About / Description -->
    {#if descriptionProps || tagsProps}
      <div class={style.aboutSection}>
        {#if descriptionProps}
          <CardDescription
            {...descriptionProps}
            className={style.description}
          />
        {/if}
      </div>
    {/if}

    <!-- Stats -->
    {#if tagsProps}
      <CardTags
        {...tagsProps}
        variant="stats"
        className={style.stats}
        itemClassName={style.statItem}
        titleClassName={style.statValue}
        descriptionClassName={style.statLabel}
      />
    {/if}
  </div>
</aside>
