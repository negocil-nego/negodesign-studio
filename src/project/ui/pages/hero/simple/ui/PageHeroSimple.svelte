<script lang="ts">
  import Menu from "../../../../organisms/menu/simple/ui/Menu.svelte";
  import { StyleComponent } from "../../../../../../components/local/data/style.component";
  import type { PageHeroSimpleProps } from "../data/types";
  import SectionText from "../../../../organisms/section/hero/shared/ui/SectionText.svelte";

  let { menuProps, sectionHeroProps }: PageHeroSimpleProps = $props();
  let heroProps = $derived(sectionHeroProps ?? {});
</script>

<header
  class={heroProps.style === "light"
    ? StyleComponent.pages.hero.light.root
    : StyleComponent.pages.hero.simple.root}
  style:background-color={heroProps.style === "light"
    ? "#eef5f6"
    : (heroProps.backgroundColor ?? "#10212b")}
>
  {#if heroProps.style === "light"}
    <div class={StyleComponent.pages.hero.light.backdrop} aria-hidden="true"></div>
  {:else if heroProps.backgroundMediaUrl}
    <div
      class={StyleComponent.pages.hero.simple.background}
      style={`background-image: url("${heroProps.backgroundMediaUrl}")`}
      aria-hidden="true"
    ></div>
  {/if}
  {#if heroProps.style !== "light"}
    <div
      class={StyleComponent.pages.hero.simple.overlay}
      aria-hidden="true"
    ></div>
    <div
      class={StyleComponent.pages.hero.simple.shade}
      style:opacity={Math.min(1, Math.max(0, heroProps.backgroundOverlayOpacity ?? 0.55))}
      aria-hidden="true"
    ></div>
  {/if}

  {#if menuProps}
    <aside class={heroProps.style === "light"
      ? StyleComponent.pages.hero.light.navigation
      : StyleComponent.pages.hero.simple.navigation}>
      <Menu
        {...menuProps}
        style={heroProps.style === "light" ? "ghost" : "ghost-inverse"}
      />
    </aside>
  {/if}

  {#if heroProps.textSectionProps}
    <SectionText {...heroProps.textSectionProps} />
  {/if}
</header>
