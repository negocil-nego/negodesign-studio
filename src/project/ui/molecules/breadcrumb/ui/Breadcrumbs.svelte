<script lang="ts">
  import { StyleComponent } from "../../../../data/style.component";
  import type { BreadcrumbItem, BreadcrumbsProps } from "../data/types";

  let {
    items = [
      { label: "Home", href: "#home" },
      { label: "Library", href: "#library" },
      { label: "Data", active: true },
    ],
    onSelect,
  }: BreadcrumbsProps = $props();

  function handleClick(item: BreadcrumbItem, idx: number, event: MouseEvent) {
    if (onSelect) {
      event.preventDefault();
      onSelect(item, idx);
    }
  }
</script>

<nav aria-label="breadcrumb" class={StyleComponent.breadcrumb.nav}>
  <ol class={StyleComponent.breadcrumb.list}>
    {#each items as item, idx (idx)}
      {@const isLast = idx === items.length - 1 || Boolean(item.active)}
      <li
        class={`${StyleComponent.breadcrumb.item} ${isLast ? StyleComponent.breadcrumb.itemActive : ""}`}
        aria-current={isLast ? "page" : undefined}
      >
        {#if !isLast && item.href}
          <a
            href={item.href}
            class={StyleComponent.breadcrumb.link}
            onclick={(event) => handleClick(item, idx, event)}
          >
            {item.label}
          </a>
        {:else}
          <span>{item.label}</span>
        {/if}
      </li>
    {/each}
  </ol>
</nav>
