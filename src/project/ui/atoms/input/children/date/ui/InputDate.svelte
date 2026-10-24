<script lang="ts">
  import Icon from "../../../../icon/ui/Icon.svelte";
  import Popover from "../../../../popover/ui/Popover.svelte";
  import { StyleComponent } from "../../../../../../data/style.component";
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
  const style = StyleComponent.input.date;
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

<div class={style.root}>
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
        class={style.trigger}
      >
        <div class={style.triggerBody}>
          {#if labelProps}
            <span class={style.triggerLabel}>
              {labelProps.label}
            </span>
          {/if}
          <div class={style.valueRow}>
            <span class={style.valueIcon}>
              <Icon name="Calendar" />
            </span>
            <span class={style.value}>{dateStr}</span>
          </div>
        </div>
      </button>
    {/snippet}

    {#snippet panel()}
      <div class={style.panel}>
        <div class={style.panelHeader}>
          <button
            type="button"
            onclick={() => changeMonth(-1)}
            class={style.navButton}
          >
            <Icon name="ArrowLeft" />
          </button>
          <span class={style.monthTitle}>{monthName} {year}</span>
          <button
            type="button"
            onclick={() => changeMonth(1)}
            class={style.navButton}
          >
            <Icon name="ArrowRight" />
          </button>
        </div>

        <div class={style.weekdays}>
          <span>Mon</span><span>Tue</span><span>Wed</span><span>Thu</span><span
            >Fri</span
          ><span>Sat</span><span>Sun</span>
        </div>

        <div class={style.daysGrid}>
          {#each paddingDays as pad}
            <span class={style.paddingDay}>{pad}</span>
          {/each}
          {#each days as d}
            <button
              type="button"
              popovertarget={popoverId}
              popovertargetaction="hide"
              onclick={() => handleSelect(d)}
              class={`${style.day} ${d === day ? style.daySelected : style.dayDefault}`}
            >
              {d}
            </button>
          {/each}
        </div>
      </div>
    {/snippet}
  </Popover>
</div>
