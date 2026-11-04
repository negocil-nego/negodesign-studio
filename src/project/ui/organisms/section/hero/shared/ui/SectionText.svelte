<script lang="ts">
  import { StyleComponent } from "../../../../../../../components/local/data/style.component";
  import Text from "../../../../../atoms/text/ui/Text.svelte";
  import Button from "../../../../../atoms/button/variant/default/ui/Button.svelte";
  import type { SectionTextProps } from "../data/types";

  let {
    titleProps,
    subtitleProps,
    descriptionProps,
    imageUrl,
    imageAlt,
    buttonsProps = [],
    layout,
  }: SectionTextProps = $props();
</script>

<section
  class={layout === "light"
    ? StyleComponent.pages.hero.light.content
    : layout === "dual"
    ? StyleComponent.pages.hero.dual.content
    : StyleComponent.pages.hero.simple.content}
>
  {#if titleProps}
    <div
      class={layout === "light"
        ? StyleComponent.pages.hero.light.eyebrow
        : layout === "dual"
        ? StyleComponent.pages.hero.dual.eyebrow
        : StyleComponent.pages.hero.simple.eyebrow}
    >
      <Text
        {...titleProps}
        className={layout === "light"
          ? StyleComponent.pages.hero.light.eyebrowText
          : layout === "dual"
          ? StyleComponent.pages.hero.dual.eyebrowText
          : StyleComponent.pages.hero.simple.eyebrowText}
      />
    </div>
  {/if}

  {#if subtitleProps}
    <div role="heading" aria-level="1">
      <Text
        {...subtitleProps}
        className={layout === "light"
          ? StyleComponent.pages.hero.light.titleText
          : layout === "dual"
          ? StyleComponent.pages.hero.dual.titleText
          : StyleComponent.pages.hero.simple.titleText}
      />
    </div>
  {/if}

  {#if descriptionProps}
    <Text
      {...descriptionProps}
      className={layout === "light"
        ? StyleComponent.pages.hero.light.descriptionText
        : layout === "dual"
        ? StyleComponent.pages.hero.dual.descriptionText
        : StyleComponent.pages.hero.simple.descriptionText}
    />
  {/if}

  {#if buttonsProps.length}
    <div class="relative z-20 mt-6 flex flex-wrap justify-center gap-3">
      {#each buttonsProps as button, index (index)}
        <Button
          {...button}
          className={button.className ?? (layout === "light" ? StyleComponent.pages.hero.light.action : undefined)}
        />
      {/each}
    </div>
  {/if}

  {#if imageUrl}
    <img
      src={imageUrl}
      alt={imageAlt ?? ""}
      class={layout === "light"
        ? StyleComponent.pages.hero.light.illustration
        : StyleComponent.pages.hero.dual.media}
    />
  {/if}
</section>
