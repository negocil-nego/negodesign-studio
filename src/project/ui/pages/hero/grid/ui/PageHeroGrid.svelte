<script lang="ts">
  import { LocalStyleComponent } from "../../../../../data/style.component";
  import Menu from "../../../../organisms/menu/simple/ui/Menu.svelte";
  import SectionCardGrid from "../../../../organisms/section/hero/grid/ui/SectionCardGrid.svelte";
  import type { PageHeroGridCardProps, PageHeroGridProps } from "../data/types";

  let {
    menuProps,
    backgroundColor = "#17161c",
    cards = [],
  }: PageHeroGridProps = $props();

  const columnSpanClasses: Record<number, string> = {
    1: "md:col-span-1",
    2: "md:col-span-2",
    3: "md:col-span-3",
    4: "md:col-span-4",
    5: "md:col-span-5",
    6: "md:col-span-6",
    7: "md:col-span-7",
    8: "md:col-span-8",
    9: "md:col-span-9",
    10: "md:col-span-10",
    11: "md:col-span-11",
    12: "md:col-span-12",
  };
  const rowSpanClasses: Record<number, string> = {
    1: "md:row-span-1",
    2: "md:row-span-2",
    3: "md:row-span-3",
    4: "md:row-span-4",
    5: "md:row-span-5",
    6: "md:row-span-6",
    7: "md:row-span-7",
  };

  function cardGridClasses(card: PageHeroGridCardProps) {
    return `${columnSpanClasses[card.colSpan ?? 12]} ${rowSpanClasses[card.rowSpan ?? 2]} ${card.className ?? ""}`;
  }
</script>

<section
  class={LocalStyleComponent.pages.hero.grid.root}
  style:background-color={backgroundColor}
  aria-label="Hero em mosaico"
>
  {#if menuProps}
    <aside class={LocalStyleComponent.pages.hero.grid.navigation}>
      <Menu {...menuProps} style={menuProps.style ?? "ghost-inverse"} />
    </aside>
  {/if}

  <div class={LocalStyleComponent.pages.hero.grid.tiles}>
    {#each cards as card (card.id)}
      <article
        class={`${LocalStyleComponent.pages.hero.grid.card} ${cardGridClasses(card)}`}
        style:background-color={card.backgroundColor ?? "#28262e"}
        aria-label={card.sectionCardProps?.subtitleProps?.text ?? card.id}
      >
        {#if card.imageUrl}
          <img
            src={card.imageUrl}
            alt={card.imageAlt ?? ""}
            loading="lazy"
            class={`${
              card.imageMode === "contain"
                ? LocalStyleComponent.pages.hero.grid.imageContain
                : LocalStyleComponent.pages.hero.grid.imageCover
            } ${card.imageClassName ?? ""}`}
          />
        {/if}

        {#if card.showOverlay}
          <div
            class={LocalStyleComponent.pages.hero.grid.overlay}
            aria-hidden="true"
          ></div>
        {/if}

        {#if card.sectionCardProps}
          <SectionCardGrid {...card.sectionCardProps} />
        {/if}
      </article>
    {/each}
  </div>
</section>
