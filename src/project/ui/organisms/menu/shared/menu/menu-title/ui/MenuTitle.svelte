<script lang="ts">
  import { StyleComponent } from "../../../../../../../data/style.component";
  import Text from "../../../../../../atoms/text/ui/Text.svelte";
  import type { MenuTitleProps } from "../data/types";

  let {
    title,
    link,
    description,
    href,
    variant = "item",
  }: MenuTitleProps = $props();
  let classes = $derived(
    variant === "featured"
      ? StyleComponent.menu.title.featured
      : StyleComponent.menu.title.item,
  );
</script>

<div class={classes.root}>
  {#if title}
    {#if href}
      <a class={classes.titleLink} {href}><Text {...title} /></a>
    {:else}
      <div class={classes.title}><Text {...title} /></div>
    {/if}
  {/if}

  {#if description}
    <div class={classes.description}><Text {...description} /></div>
  {/if}

  {#if link}
    {#if variant === "featured" && href}
      <a class={classes.link} {href}
        ><Text {...link} /><span aria-hidden="true">→</span></a
      >
    {:else}
      <div class={classes.link}><Text {...link} /></div>
    {/if}
  {/if}
</div>
