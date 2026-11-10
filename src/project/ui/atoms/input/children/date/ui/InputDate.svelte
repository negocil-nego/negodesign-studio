<script lang="ts">
  import Icon from "../../../../icon/ui/Icon.svelte";
  import Popover from "../../../../popover/ui/Popover.svelte";
  import { StyleComponent } from "../../../../../../data/style.component";
  import { locale, t } from "../../../../../../data/i18n";
  import {
    computeCalendarMonth,
    formatLongDate,
    stepCalendarMonth,
  } from "../../../../../../hooks/date-picker.hook";
  import type { InputDateProps } from "../data/types";

  let {
    labelProps,
    inputProps,
    value = $bindable(formatLongDate($locale, 2023, 10, 21)),
    onselect,
  }: InputDateProps & { value?: string; onselect?: (date: string) => void } = $props();

  const popoverId = $props.id();
  const style = StyleComponent.input.date;
  let year = $state(2023);
  let month = $state(10);
  let day = $state(21);
  let dateStr = $state(value);

  const cal = $derived(computeCalendarMonth($locale, year, month));

  function changeMonth(delta: number) {
    const next = stepCalendarMonth(year, month, delta);
    year = next.year;
    month = next.month;
  }

  function handleSelect(selectedDay: number) {
    day = selectedDay;
    dateStr = formatLongDate($locale, year, month, day);
    onselect?.(dateStr);
  }
</script>

<div class={style.root}>
  <Popover id={popoverId} role="dialog" ariaLabel={labelProps?.label ?? $t("input.date.selectDate")}>
    {#snippet trigger()}
      <button type="button" id={inputProps?.id} disabled={inputProps?.disabled} popovertarget={popoverId}
        aria-haspopup="dialog" class={style.trigger}>
        <div class={style.triggerBody}>
          {#if labelProps}<span class={style.triggerLabel}>{labelProps.label}</span>{/if}
          <div class={style.valueRow}>
            <span class={style.valueIcon}><Icon name="Calendar" /></span>
            <span class={style.value}>{dateStr}</span>
          </div>
        </div>
      </button>
    {/snippet}

    {#snippet panel()}
      <div class={style.panel}>
        <div class={style.panelHeader}>
          <button type="button" onclick={() => changeMonth(-1)} class={style.navButton}><Icon name="ArrowLeft" /></button>
          <span class={style.monthTitle}>{cal.monthName} {year}</span>
          <button type="button" onclick={() => changeMonth(1)} class={style.navButton}><Icon name="ArrowRight" /></button>
        </div>
        <div class={style.weekdays}>
          {#each cal.weekdays as wd}<span>{wd}</span>{/each}
        </div>
        <div class={style.daysGrid}>
          {#each cal.paddingDays as pad}<span class={style.paddingDay}>{pad}</span>{/each}
          {#each cal.days as d}
            <button type="button" popovertarget={popoverId} popovertargetaction="hide"
              onclick={() => handleSelect(d)}
              class={`${style.day} ${d === day ? style.daySelected : style.dayDefault}`}>{d}</button>
          {/each}
        </div>
      </div>
    {/snippet}
  </Popover>
</div>
