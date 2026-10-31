<script lang="ts">
  import { StyleComponent } from "../../../../../../data/style.component";
  import { getCarouselBackground } from "../data/background";
  import type { SectionImageCarouselProps } from "../data/types";

  let { items = [], className = "", backgroundColor = "" }: SectionImageCarouselProps = $props();
  let activeIndex = $state(0);
  let currentIndex = $derived(items.length ? Math.min(activeIndex, items.length - 1) : 0);
  let activeItem = $derived(items[currentIndex]);
  let background = $derived(getCarouselBackground(backgroundColor));

  function showSlide(index: number) {
    if (items.length === 0) return;
    activeIndex = (index + items.length) % items.length;
  }
</script>

{#if activeItem}
  <section
    class={`${StyleComponent.carousel.root} ${background.backgroundClass} ${className}`.trim()}
    style={background.isCssColor ? `background-color: ${backgroundColor}` : undefined}
    aria-label="Carousel"
    aria-roledescription="carousel"
  >
    <div
      class={StyleComponent.carousel.slide}
      aria-live="polite"
      aria-atomic="true"
      role="group"
      aria-roledescription="slide"
      aria-label={`Slide ${currentIndex + 1} de ${items.length}`}
    >
      {#if activeItem.backgroundImage}
        <div
          class={StyleComponent.carousel.backgroundImage}
          style={`background-image: url("${activeItem.backgroundImage}")`}
          aria-hidden="true"
        ></div>
      {/if}
      {#if background.isCssGradient || background.isGradientClass}
        <div
          class={`${StyleComponent.carousel.backgroundGradient} ${background.backgroundGradientClass}`.trim()}
          style={background.isCssGradient ? `background-image: ${backgroundColor}` : undefined}
          aria-hidden="true"
        ></div>
      {/if}
      <div class={StyleComponent.carousel.overlay} aria-hidden="true"></div>
      <div class={StyleComponent.carousel.content}>
        {#if activeItem.title?.text}
          <h2 class={StyleComponent.carousel.title}>{activeItem.title.text}</h2>
        {/if}
        {#if activeItem.description?.text}
          <p class={StyleComponent.carousel.description}>
            {activeItem.description.text}
          </p>
        {/if}
      </div>

      {#if items.length > 1}
        <div class={StyleComponent.carousel.indicators} aria-label="Slides">
          {#each items as _, index (index)}
            <button
              type="button"
              class={`${StyleComponent.carousel.indicator} ${currentIndex === index ? StyleComponent.carousel.indicatorActive : ""}`}
              aria-label={`Mostrar slide ${index + 1}`}
              aria-current={currentIndex === index ? "true" : undefined}
              onclick={() => showSlide(index)}
            ></button>
          {/each}
        </div>
      {/if}
    </div>
  </section>
{/if}
