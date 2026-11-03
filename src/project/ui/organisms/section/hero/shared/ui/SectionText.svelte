<script lang="ts">
  import { StyleComponent } from "../../../../../../../components/local/data/style.component";
  import Text from "../../../../../atoms/text/ui/Text.svelte";
  import type { SectionTextProps } from "../data/types";

  let { titleProps, subtitleProps, descriptionProps }: SectionTextProps =
    $props();
</script>

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
