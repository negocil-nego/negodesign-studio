<script lang="ts">
  import Icon from "../../../../icon/ui/Icon.svelte";
  import Popover from "../../../../popover/ui/Popover.svelte";
  import type { InputDateProps } from "../data/types";

  let {
    labelProps,
    inputProps,
    value = $bindable("November 21, 2023"),
    onselect,
  }: InputDateProps & {
    value?: string;
    onselect?: (date: string) => void;
  } = $props();

  const popoverId = $props.id();
  let year = $state(2023);
  let month = $state(10); // 10 = November
  let day = $state(21);
  let dateStr = $state(value);

  const months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];

  const monthName = $derived(months[month]);
  const daysInMonth = $derived(new Date(year, month + 1, 0).getDate());
  const startDay = $derived((new Date(year, month, 1).getDay() + 6) % 7);
  const days = $derived(Array.from({ length: daysInMonth }, (_, i) => i + 1));
  const prevDaysCount = $derived(new Date(year, month, 0).getDate());
  const paddingDays = $derived(
    Array.from(
      { length: startDay },
      (_, i) => prevDaysCount - startDay + i + 1,
    ),
  );

  function changeMonth(delta: number) {
    const next = month + delta;
    if (next > 11) {
      month = 0;
      year++;
    } else if (next < 0) {
      month = 11;
      year--;
    } else {
      month = next;
    }
  }

  function handleSelect(selectedDay: number) {
    day = selectedDay;
    dateStr = `${monthName} ${day}, ${year}`;
    onselect?.(dateStr);
  }
</script>

<div class="relative w-full">
  <Popover
    id={popoverId}
    role="dialog"
    ariaLabel={labelProps?.label ?? "Pick a date"}
  >
    {#snippet trigger()}
      <button
        type="button"
        id={inputProps?.id}
        disabled={inputProps?.disabled}
        popovertarget={popoverId}
        aria-haspopup="dialog"
        class="input-form w-full cursor-pointer flex items-center justify-between text-left"
      >
        <div class="flex flex-col w-full min-w-0">
          {#if labelProps}
            <span
              class="text-[10px] uppercase font-semibold text-zinc-500 select-none"
            >
              {labelProps.label}
            </span>
          {/if}
          <div class="flex items-center gap-2 mt-0.5">
            <span class="text-sm">
              <Icon name="Label" />
            </span>
            <span class="text-sm font-medium truncate">{dateStr}</span>
          </div>
        </div>
        <div
          class="flex flex-col text-zinc-400 text-xs ml-2 select-none shrink-0"
          aria-hidden="true"
        >
          <span><Icon name="HelpSquare" /></span>
          <span><Icon name="HelpSquare" /></span>
        </div>
      </button>
    {/snippet}

    {#snippet panel()}
      <div class="mt-1.5 text-zinc-200 z-50 outline-none">
        <div class="flex items-center justify-between mb-4">
          <button
            type="button"
            onclick={() => changeMonth(-1)}
            class="p-1.5 rounded hover:bg-blender-hover cursor-pointer"
          >
            <Icon name="HelpSquare" />
          </button>
          <span class="font-bold text-sm text-zinc-200">{monthName} {year}</span
          >
          <button
            type="button"
            onclick={() => changeMonth(1)}
            class="p-1.5 rounded hover:bg-blender-hover cursor-pointer"
          >
            <Icon name="HelpSquare" />
          </button>
        </div>

        <div
          class="grid grid-cols-7 text-center text-xs font-medium text-zinc-500 mb-3"
        >
          <span>Mon</span><span>Tue</span><span>Wed</span><span>Thu</span><span
            >Fri</span
          ><span>Sat</span><span>Sun</span>
        </div>

        <div class="grid grid-cols-7 gap-y-1.5 text-center text-sm font-medium">
          {#each paddingDays as pad}
            <span class="text-zinc-600 py-1 opacity-40">{pad}</span>
          {/each}
          {#each days as d}
            <button
              type="button"
              popovertarget={popoverId}
              popovertargetaction="hide"
              onclick={() => handleSelect(d)}
              class="w-8 h-8 mx-auto flex items-center justify-center rounded-full transition-all cursor-pointer
                {d === day
                ? 'bg-orange-500 text-white font-bold shadow-md'
                : 'text-zinc-300 hover:bg-blender-hover'}"
            >
              {d}
            </button>
          {/each}
        </div>
      </div>
    {/snippet}
  </Popover>
</div>
