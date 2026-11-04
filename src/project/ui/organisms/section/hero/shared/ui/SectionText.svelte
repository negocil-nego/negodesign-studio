<script lang="ts">
  import { StyleComponent } from "../../../../../../../components/local/data/style.component";
  import Text from "../../../../../atoms/text/ui/Text.svelte";
  import type { SectionTextProps } from "../data/types";

  let {
    titleProps,
    subtitleProps,
    descriptionProps,
    imageUrl,
    imageAlt,
    layout,
  }: SectionTextProps = $props();
</script>

<section
  class={layout === "dual"
    ? StyleComponent.pages.hero.dual.content
    : StyleComponent.pages.hero.simple.content}
>
  {#if titleProps}
    <div
      class={layout === "dual"
        ? StyleComponent.pages.hero.dual.eyebrow
        : StyleComponent.pages.hero.simple.eyebrow}
    >
      <Text
        {...titleProps}
        className={layout === "dual"
          ? StyleComponent.pages.hero.dual.eyebrowText
          : StyleComponent.pages.hero.simple.eyebrowText}
      />
    </div>
  {/if}

  {#if subtitleProps}
    <div role="heading" aria-level="1">
      <Text
        {...subtitleProps}
        className={layout === "dual"
          ? StyleComponent.pages.hero.dual.titleText
          : StyleComponent.pages.hero.simple.titleText}
      />
    </div>
  {/if}

  {#if descriptionProps}
    <Text
      {...descriptionProps}
      className={layout === "dual"
        ? StyleComponent.pages.hero.dual.descriptionText
        : StyleComponent.pages.hero.simple.descriptionText}
    />
  {/if}

  {#if imageUrl}
    <img
      src={imageUrl}
      alt={imageAlt ?? ""}
      class={StyleComponent.pages.hero.dual.media}
    />
  {/if}
</section>
