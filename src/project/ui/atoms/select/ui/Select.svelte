<script lang="ts">
  import type { HTMLButtonAttributes } from "svelte/elements";
  import Icon from "../../icon/ui/Icon.svelte";
  import Popover from "../../popover/ui/Popover.svelte";
  import { StyleComponent } from "../../../../data/style.component";
  import type { SelectProps, SelectOption } from "../data/types";

  type Props = SelectProps & Omit<HTMLButtonAttributes, keyof SelectProps | "type" | "class"> & { class?: string };
  let {
    id, name, label, options, value = $bindable(""), placeholder, disabled,
    required, onOptionSelect, onclick: onNativeClick, class: triggerClass, ...rest
  }: Props = $props();

  const popoverId = $props.id();
  let query = $state("");
  const selected = $derived(options.find((opt) => opt.value === value || opt.label === value));
  const filtered = $derived.by(() => {
    const term = query.trim().toLowerCase();
    if (!term) return options;
    return options.filter((opt) => opt.label.toLowerCase().includes(term));
  });

  function choose(option: SelectOption) {
    value = option.value;
    onOptionSelect?.(option);
    query = "";
  }

  function handleTriggerClick(event: MouseEvent) {
    onNativeClick?.(event);
  }
</script>

<div class={StyleComponent.select.root}>
  <Popover id={popoverId} role="listbox" ariaLabel={label ?? "Select option"}>
    {#snippet trigger()}
      <button
        {...rest}
        {id}
        {disabled}
        {name}
        type="button"
        popovertarget={popoverId}
        aria-haspopup="listbox"
        aria-required={required}
        onclick={handleTriggerClick}
        class={`${StyleComponent.select.trigger} ${triggerClass ?? ""}`}
      >
        {#if label}<div class={StyleComponent.select.triggerLabel}>{label}</div>
        {:else}<div class={StyleComponent.select.triggerValue}>{selected ? selected.label : (placeholder ?? "Selecione...")}</div>{/if}
        <div><Icon name="ArrowDown" /></div>
      </button>
    {/snippet}
    {#snippet panel()}
      <div class={StyleComponent.select.panel}>
        <input type="search" bind:value={query} placeholder="Search..." class={StyleComponent.select.search} />
        <ul class={StyleComponent.select.list}>
          {#each filtered as option (option.value)}
            <li>
              <button type="button" role="option" aria-selected={option.value === value || option.label === value} disabled={option.disabled} popovertarget={popoverId} popovertargetaction="hide" onclick={() => choose(option)} class={`${StyleComponent.select.option} ${selected?.value === option.value ? StyleComponent.select.optionSelected : ""}`}>
                <div class={StyleComponent.select.optionContent}>
                  {#if option.icon}<span class={StyleComponent.select.optionIcon}>{option.icon}</span>{/if}
                  <span class="truncate">{option.label}</span>
                </div>
                {#if selected?.value === option.value || selected?.label === option.label}<span class={StyleComponent.select.optionCheck}>✓</span>{/if}
              </button>
            </li>
          {:else}<li class={StyleComponent.select.optionEmpty}>No options found</li>{/each}
        </ul>
      </div>
    {/snippet}
  </Popover>
  <input type="hidden" {name} value={selected?.value ?? ""} />
</div>