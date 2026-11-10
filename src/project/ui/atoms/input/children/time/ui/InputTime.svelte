<script lang="ts">
  import Icon from "../../../../icon/ui/Icon.svelte";
  import Popover from "../../../../popover/ui/Popover.svelte";
  import { StyleComponent } from "../../../../../../data/style.component";
  import { t } from "../../../../../../data/i18n";
  import { formatTimeDisplay, parseTimeValue, TIME_HOURS, TIME_MINUTES, TIME_PERIODS } from "../../../../../../hooks/time-picker.hook";
  import type { InputTimeProps } from "../data/types";

  let { labelProps, inputProps, value = $bindable(""), onselect }: InputTimeProps & {
    value?: string;
    onselect?: (time: string) => void;
  } = $props();

  const popoverId = $props.id();
  const style = StyleComponent.input.time;
  let selectedHour = $state("02");
  let selectedMinute = $state("30");
  let selectedPeriod = $state("AM");

  $effect(() => {
    if (value) {
      const parsed = parseTimeValue(value);
      selectedHour = parsed.hour;
      selectedMinute = parsed.minute;
      selectedPeriod = parsed.period;
    }
  });

  const displayTime = $derived(formatTimeDisplay(selectedHour, selectedMinute, selectedPeriod));

  function apply(h = selectedHour, m = selectedMinute, p = selectedPeriod) {
    selectedHour = h;
    selectedMinute = m;
    selectedPeriod = p;
    value = formatTimeDisplay(h, m, p);
    onselect?.(value);
  }

  function clearTime(e: MouseEvent) {
    e.stopPropagation();
    selectedHour = "";
    selectedMinute = "";
    selectedPeriod = "AM";
    value = "";
    onselect?.("");
  }
</script>

<div class={style.root}>
  <Popover id={popoverId} role="dialog" ariaLabel={labelProps?.label ?? $t("input.time.selectTime")}>
    {#snippet trigger()}
      <div class="relative w-full">
        <button type="button" id={inputProps?.id} disabled={inputProps?.disabled} popovertarget={popoverId}
          aria-haspopup="dialog" class="{style.trigger} {value ? 'pr-10' : ''}">
          <div class={style.triggerBody}>
            {#if labelProps}<span class={style.triggerLabel}>{labelProps.label}</span>{/if}
            <div class={style.valueRow}>
              <span class={style.valueIcon}><Icon name="Time" /></span>
              <span class={style.value}>{displayTime}</span>
            </div>
          </div>
        </button>
        {#if value}
          <button type="button" class="{style.clearButton} absolute right-2 top-1/2 -translate-y-1/2"
            onclick={clearTime} aria-label={$t("input.time.clear")}><Icon name="Cancel" /></button>
        {/if}
      </div>
    {/snippet}
    {#snippet panel()}
      <div class={style.panel}>
        <div class={style.pickerGrid}>
          {#each [[TIME_HOURS, selectedHour, (h: string) => apply(h, selectedMinute, selectedPeriod)], [TIME_MINUTES, selectedMinute, (m: string) => apply(selectedHour, m, selectedPeriod)], [TIME_PERIODS, selectedPeriod, (p: string) => apply(selectedHour, selectedMinute, p)]] as [items, active, selectFn], idx}
            {#if idx > 0}<div class={style.columnDivider}></div>{/if}
            <div class={style.pickerColumn}>
              {#each items as item}
                <button type="button" class={`${style.pickerItem} ${active === item ? style.pickerItemSelected : ""}`}
                  onclick={() => selectFn(item)}>{item}</button>
              {/each}
            </div>
          {/each}
        </div>
      </div>
    {/snippet}
  </Popover>
</div>
