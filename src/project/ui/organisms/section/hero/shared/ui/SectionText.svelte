<script lang="ts">
  import { LocalStyleComponent } from "../../../../../../../components/local/data/style.component";
  import Button from "../../../../../atoms/button/variant/default/ui/Button.svelte";
  import Text from "../../../../../atoms/text/ui/Text.svelte";
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
    ? LocalStyleComponent.pages.hero.light.content
    : layout === "dual"
      ? LocalStyleComponent.pages.hero.dual.content
      : LocalStyleComponent.pages.hero.simple.content}
>
  {#if titleProps}
    <div
      class={layout === "light"
        ? LocalStyleComponent.pages.hero.light.eyebrow
        : layout === "dual"
          ? LocalStyleComponent.pages.hero.dual.eyebrow
          : LocalStyleComponent.pages.hero.simple.eyebrow}
    >
      <Text
        {...titleProps}
        className={layout === "light"
          ? LocalStyleComponent.pages.hero.light.eyebrowText
          : layout === "dual"
            ? LocalStyleComponent.pages.hero.dual.eyebrowText
            : LocalStyleComponent.pages.hero.simple.eyebrowText}
      />
    </div>
  {/if}

  {#if subtitleProps}
    <div role="heading" aria-level="1">
      <Text
        {...subtitleProps}
        className={layout === "light"
          ? LocalStyleComponent.pages.hero.light.titleText
          : layout === "dual"
            ? LocalStyleComponent.pages.hero.dual.titleText
            : LocalStyleComponent.pages.hero.simple.titleText}
      />
    </div>
  {/if}

  {#if descriptionProps}
    <Text
      {...descriptionProps}
      className={layout === "light"
        ? LocalStyleComponent.pages.hero.light.descriptionText
        : layout === "dual"
          ? LocalStyleComponent.pages.hero.dual.descriptionText
          : LocalStyleComponent.pages.hero.simple.descriptionText}
    />
  {/if}

  {#if buttonsProps.length}
    <div class="relative z-20 mt-6 flex flex-wrap justify-center gap-3">
      {#each buttonsProps as button, index (index)}
        <Button
          {...button}
          className={button.className ??
            (layout === "light"
              ? LocalStyleComponent.pages.hero.light.action
              : undefined)}
        />
      {/each}
    </div>
  {/if}

  {#if imageUrl}
    <img
      src={imageUrl}
      alt={imageAlt ?? ""}
      class={layout === "light"
        ? LocalStyleComponent.pages.hero.light.illustration
        : LocalStyleComponent.pages.hero.dual.media}
    />
  {/if}
</section>
