<script lang="ts">
  import { LocalStyleComponent } from "../../../../../../components/local/data/style.component";
  import Menu from "../../../../organisms/menu/simple/ui/Menu.svelte";
  import SectionText from "../../../../organisms/section/hero/shared/ui/SectionText.svelte";
  import type { PageHeroDualProps } from "../data/types";

  let {
    menuProps,
    leftSectionHereProps,
    rightSectionHereProps,
  }: PageHeroDualProps = $props();
</script>

<header class={LocalStyleComponent.pages.hero.dual.root}>
  <div
    class={LocalStyleComponent.pages.hero.dual.backdrop}
    aria-hidden="true"
  ></div>
  {#if menuProps}
    <aside class={LocalStyleComponent.pages.hero.dual.navigation}>
      <Menu {...menuProps} style={menuProps.style ?? "ghost"} />
    </aside>
  {/if}

  <div class={LocalStyleComponent.pages.hero.dual.layout}>
    {#if leftSectionHereProps}
      <div class={LocalStyleComponent.pages.hero.dual.section}>
        {#if leftSectionHereProps.textSectionProps}
          <SectionText {...leftSectionHereProps.textSectionProps} />
        {/if}
      </div>
    {/if}
    {#if rightSectionHereProps}
      <div class={LocalStyleComponent.pages.hero.dual.section}>
        {#if rightSectionHereProps.textSectionProps}
          <SectionText {...rightSectionHereProps.textSectionProps} />
        {/if}
      </div>
    {/if}
  </div>
</header>
