<script lang="ts">
  import { StyleComponent } from "../../../../data/style.component";
  import type { UnderlineNavProps } from "../data/types";

  let {
    items = [
      { value: "active", label: "Active" },
      { value: "link-1", label: "Link" },
      { value: "link-2", label: "Link" },
      { value: "disabled", label: "Disabled", disabled: true },
    ],
    value = $bindable("active"),
    onValueChange,
  }: UnderlineNavProps = $props();

  function select(next: string, disabled?: boolean) {
    if (disabled) return;
    value = next;
    onValueChange?.(value);
  }
</script>

<ul class={StyleComponent.underlineNav.root} role="tablist">
  {#each items as item (item.value)}
    {@const active = item.value === value}
    <li class={StyleComponent.underlineNav.item}>
      <button
        type="button"
        role="tab"
        aria-selected={active}
        disabled={item.disabled}
        onclick={() => select(item.value, item.disabled)}
        class={`
          ${StyleComponent.underlineNav.link}
          ${active ? StyleComponent.underlineNav.linkActive : ""}
          ${item.disabled ? StyleComponent.underlineNav.linkDisabled : ""}
        `}
      >
        {item.label}
      </button>
    </li>
  {/each}
</ul>
