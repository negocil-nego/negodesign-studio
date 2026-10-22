<script lang="ts">
  import Icon from "../../icon/ui/Icon.svelte";
  import Popover from "../../popover/ui/Popover.svelte";
  import type { SelectProps, SelectOption } from "../data/types";

  let {
    id,
    name,
    label,
    options,
    value = $bindable(""),
    placeholder,
    disabled,
    required,
    onselect,
  }: SelectProps = $props();

  const popoverId = $props.id();
  let query = $state("");

  const selected = $derived(
    options.find((opt) => opt.value === value || opt.label === value),
  );

  const filtered = $derived.by(() => {
    const term = query.trim().toLowerCase();
    if (!term) return options;
    return options.filter((opt) => opt.label.toLowerCase().includes(term));
  });

  function choose(opt: SelectOption) {
    value = opt.label;
    onselect?.(opt);
    query = "";
  }
</script>

<div class="relative w-full">
  <Popover id={popoverId} role="listbox" ariaLabel={label ?? "Select option"}>
    {#snippet trigger()}
      <button
        {id}
        {disabled}
        type="button"
        popovertarget={popoverId}
        aria-haspopup="listbox"
        class="w-full flex justify-center items-center text-left"
      >
        {#if label}
          <div
            class="text-[10px] uppercase font-semibold text-zinc-500 select-none w-full"
          >
            {label}
          </div>
        {:else}
          <div class="text-sm truncate w-full">
            {selected ? selected.label : (placeholder ?? "Selecione...")}
          </div>
        {/if}
        <div>
          <Icon name="Home" />
        </div>
      </button>
    {/snippet}

    {#snippet panel()}
      <div class="mt-1.5 p-2 w-full min-w-60 shadow-lg z-50">
        <input
          type="search"
          bind:value={query}
          placeholder="Search..."
          class="w-full px-3 py-1.5 mb-1.5 text-sm bg-blender-surface border border-blender-border rounded text-inherit outline-none focus:border-blue-500"
        />
        <ul class="max-h-56 overflow-y-auto flex flex-col gap-0.5">
          {#each filtered as option (option.value)}
            <li>
              <button
                type="button"
                role="option"
                aria-selected={option.value === value || option.label === value}
                disabled={option.disabled}
                popovertarget={popoverId}
                popovertargetaction="hide"
                onclick={() => choose(option)}
                class="w-full flex items-center justify-between px-3 py-2 rounded text-sm text-zinc-300 hover:text-slate-900 hover:bg-blender-hover active:bg-blender-active transition-colors cursor-pointer text-left
                  {selected?.value === option.value
                  ? 'bg-blender-hover text-slate-900 font-medium'
                  : ''}"
              >
                <div class="flex items-center gap-2.5 truncate">
                  {#if option.icon}
                    <span class="text-base">{option.icon}</span>
                  {/if}
                  <span class="truncate">{option.label}</span>
                </div>
                {#if selected?.value === option.value || selected?.label === option.label}
                  <span class="text-blue-500 font-bold text-xs">✓</span>
                {/if}
              </button>
            </li>
          {:else}
            <li class="px-3 py-2 text-xs text-zinc-500 text-center select-none">
              No options found
            </li>
          {/each}
        </ul>
      </div>
    {/snippet}
  </Popover>
</div>
