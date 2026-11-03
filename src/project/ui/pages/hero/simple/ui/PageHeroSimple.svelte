<script lang="ts">
  import Text from "../../../../atoms/text/ui/Text.svelte";
  import Menu from "../../../../organisms/menu/simple/ui/Menu.svelte";
  import { StyleComponent } from "../../../../../../components/local/data/style.component";
  import type { PageHeroSimpleProps } from "../data/types";

  let {
    menuProps,
    titleProps,
    subtitleProps,
    descriptionProps,
    backgroundColor,
    backgroundMediaUrl,
  }: PageHeroSimpleProps = $props();
</script>

<header
  class={StyleComponent.pages.hero.simple.root}
  style:background-color={backgroundColor ?? "#10212b"}
>
  {#if backgroundMediaUrl}
    <div
      class={StyleComponent.pages.hero.simple.background}
      style={`background-image: url("${backgroundMediaUrl}")`}
      aria-hidden="true"
    ></div>
    <div class="absolute h-screen w-screen bg-black/10"></div>
  {/if}
  <div
    class={StyleComponent.pages.hero.simple.overlay}
    aria-hidden="true"
  ></div>

  {#if menuProps}
    <aside class={StyleComponent.pages.hero.simple.navigation}>
      <Menu {...menuProps} />
    </aside>
  {/if}

  <section class={StyleComponent.pages.hero.simple.content}>
    {#if titleProps}
      <div class={StyleComponent.pages.hero.simple.eyebrow}>
        <Text
          {...titleProps}
          className={StyleComponent.pages.hero.simple.eyebrowText}
        />
      </div>
    {/if}

    {#if subtitleProps}
      <div role="heading" aria-level="1">
        <Text
          {...subtitleProps}
          className={StyleComponent.pages.hero.simple.titleText}
        />
      </div>
    {/if}

    {#if descriptionProps}
      <Text
        {...descriptionProps}
        className={StyleComponent.pages.hero.simple.descriptionText}
      />
    {/if}
  </section>
</header>
