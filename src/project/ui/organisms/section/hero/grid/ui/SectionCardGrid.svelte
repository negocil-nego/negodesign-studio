<script lang="ts">
  import { StyleComponent } from "../../../../../../../components/local/data/style.component";
  import Button from "../../../../../atoms/button/variant/default/ui/Button.svelte";
  import Text from "../../../../../atoms/text/ui/Text.svelte";
  import type { SectionCardGridProps } from "../data/types";

  let {
    titleProps,
    subtitleProps,
    descriptionProps,
    buttonsProps = [],
    tone = "dark",
    contentPosition = "end",
  }: SectionCardGridProps = $props();

  let styles = $derived(
    tone === "light"
      ? StyleComponent.pages.hero.gridLight
      : StyleComponent.pages.hero.grid,
  );
  const positionClasses = {
    start: "justify-start",
    center: "justify-center",
    end: "justify-end",
    between: "justify-between",
  };
</script>

<section class={`${styles.content} ${positionClasses[contentPosition]}`}>
  {#if titleProps}
    <div class={styles.eyebrow}>
      <Text {...titleProps} className={styles.eyebrowText} />
    </div>
  {/if}

  {#if subtitleProps}
    <div role="heading" aria-level="2">
      <Text {...subtitleProps} className={styles.titleText} />
    </div>
  {/if}

  {#if descriptionProps}
    <Text {...descriptionProps} className={styles.descriptionText} />
  {/if}

  {#if buttonsProps.length}
    <div class="relative z-20 mt-4 flex flex-wrap justify-start gap-3">
      {#each buttonsProps as button, index (index)}
        <Button
          {...button}
          className={button.className ?? styles.action}
        />
      {/each}
    </div>
  {/if}
</section>
