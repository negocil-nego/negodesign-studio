<script lang="ts">
  import Menu from "../../../../organisms/menu/simple/ui/Menu.svelte";
  import { StyleComponent } from "../../../../../../components/local/data/style.component";
  import type { PageHeroSimpleProps } from "../data/types";
  import SectionText from "../../../../organisms/section/hero/shared/ui/SectionText.svelte";

  let { menuProps, sectionHeroProps }: PageHeroSimpleProps = $props();
  let heroProps = $derived(sectionHeroProps ?? {});
</script>

<header
  class={StyleComponent.pages.hero.simple.root}
  style:background-color={heroProps.backgroundColor ?? "#10212b"}
>
  {#if heroProps.backgroundMediaUrl}
    <div
      class={StyleComponent.pages.hero.simple.background}
      style={`background-image: url("${heroProps.backgroundMediaUrl}")`}
      aria-hidden="true"
    ></div>
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

  {#if heroProps.textSectionProps}
    <SectionText {...heroProps.textSectionProps} />
  {/if}
</header>
