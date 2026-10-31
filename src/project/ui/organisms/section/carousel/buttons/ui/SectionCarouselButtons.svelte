<script lang="ts">
  import Icon from "../../../../../atoms/icon/ui/Icon.svelte";
  import { StyleComponent } from "../../../../../../data/style.component";
  import type { SectionCarouselButtonsProps } from "../data/types";

  let { titleProps, descriptionProps, className = "", children }: SectionCarouselButtonsProps = $props();
  let viewport = $state<HTMLDivElement>();

  function scroll(direction: -1 | 1) {
    viewport?.scrollBy({
      left: direction * viewport.clientWidth * 0.8,
      behavior: "smooth",
    });
  }
</script>

<section class={`${StyleComponent.sectionCarouselButtons.root} ${className}`.trim()}>
  <header class={StyleComponent.sectionCarouselButtons.header}>
    <div class={StyleComponent.sectionCarouselButtons.heading}>
      {#if titleProps}<h2 class={StyleComponent.sectionCarouselButtons.title}>{titleProps.text}</h2>{/if}
      {#if descriptionProps}<p class={StyleComponent.sectionCarouselButtons.description}>{descriptionProps.text}</p>{/if}
    </div>
    <div class={StyleComponent.sectionCarouselButtons.controls}>
      <button type="button" class={StyleComponent.sectionCarouselButtons.control} aria-label="Itens anteriores" onclick={() => scroll(-1)}>
        <Icon name="ArrowLeft" className="size-4" />
      </button>
      <button type="button" class={StyleComponent.sectionCarouselButtons.control} aria-label="Itens seguintes" onclick={() => scroll(1)}>
        <Icon name="ArrowRight" className="size-4" />
      </button>
    </div>
  </header>

  <div class={StyleComponent.sectionCarouselButtons.viewport}>
    <div bind:this={viewport} class={StyleComponent.sectionCarouselButtons.track}>
      {@render children()}
    </div>
  </div>
</section>
