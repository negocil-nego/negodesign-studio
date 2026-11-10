<script lang="ts">
  import { StyleComponent } from "../../../../data/style.component";
  import type { ListGroupEntry, ListGroupsProps } from "../data/types";

  const defaultItems: ListGroupEntry[] = [
    { id: "item-1", label: "Cras justo odio", badge: 14, active: true },
    { id: "item-2", label: "Dapibus ac facilisis in", badge: 2 },
    { id: "item-3", label: "Morbi leo risus", badge: 1 },
  ];

  let {
    items = defaultItems,
    activeId = $bindable<string | undefined>(undefined),
    onSelect,
  }: ListGroupsProps = $props();

  function isActive(item: ListGroupEntry): boolean {
    return activeId !== undefined ? activeId === item.id : Boolean(item.active);
  }

  function handleSelect(item: ListGroupEntry) {
    if (item.disabled) return;
    activeId = item.id;
    onSelect?.(item.id);
  }
</script>

<ul class={StyleComponent.listGroup.root}>
  {#each items as item (item.id)}
    {@const active = isActive(item)}
    <li>
      <button
        type="button"
        disabled={item.disabled}
        onclick={() => handleSelect(item)}
        class={`
          ${StyleComponent.listGroup.item}
          ${StyleComponent.listGroup.action}
          ${active ? StyleComponent.listGroup.active : ""}
          ${item.disabled ? StyleComponent.listGroup.disabled : ""}
        `}
      >
        {#if item.description}
          <div class="w-full text-left">
            <div class={StyleComponent.listGroup.richHeader}>
              <span class={StyleComponent.listGroup.richTitle}>{item.label}</span>
              {#if item.meta}
                <span class={StyleComponent.listGroup.richMeta}>{item.meta}</span>
              {/if}
            </div>
            <p class={StyleComponent.listGroup.richBody}>{item.description}</p>
            {#if item.footer}
              <small class={StyleComponent.listGroup.richFooter}>{item.footer}</small>
            {/if}
          </div>
        {:else}
          <span>{item.label}</span>
          {#if item.badge !== undefined}
            <span class="badge rounded-pill text-bg-primary">{item.badge}</span>
          {/if}
        {/if}
      </button>
    </li>
  {/each}
</ul>
