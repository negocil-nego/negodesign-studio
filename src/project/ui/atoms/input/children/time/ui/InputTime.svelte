<script lang="ts">
  import Icon from "../../../../icon/ui/Icon.svelte";
  import Popover from "../../../../popover/ui/Popover.svelte";
  import { StyleComponent } from "../../../../../../data/style.component";
  import { t } from "../../../../../../data/i18n";
  import type { InputTimeProps } from "../data/types";

  let {
    labelProps,
    inputProps,
    value = $bindable(""),
    onselect,
  }: InputTimeProps & {
    value?: string;
    onselect?: (time: string) => void;
  } = $props();

  const popoverId = $props.id();
  const style = StyleComponent.input.time;

  let selectedHour = $state("02");
  let selectedMinute = $state("30");
  let selectedPeriod = $state("AM");

  // Parse initial value if provided
  $effect(() => {
    if (value) {
      const match = value.match(/(\d{2}):(\d{2})\s?(AM|PM)/i);
      if (match) {
        selectedHour = match[1];
        selectedMinute = match[2];
        selectedPeriod = match[3].toUpperCase();
      }
    }
  });

  const displayTime = $derived(
    selectedHour && selectedMinute
      ? `${selectedHour}:${selectedMinute} ${selectedPeriod}`
      : "-- : --",
  );

  function handleSelect() {
    value = displayTime;
    onselect?.(value);
  }

  function clearTime(e: MouseEvent) {
    e.stopPropagation();
    value = "";
    selectedHour = "";
    selectedMinute = "";
    selectedPeriod = "AM";
    onselect?.("");
  }

  const hours = Array.from({ length: 12 }, (_, i) =>
    (i + 1).toString().padStart(2, "0"),
  );
  const minutes = Array.from({ length: 60 / 5 }, (_, i) =>
    (i * 5).toString().padStart(2, "0"),
  );
  const periods = ["AM", "PM"];

  function setHour(h: string) {
    selectedHour = h;
    handleSelect();
  }

  function setMinute(m: string) {
    selectedMinute = m;
    handleSelect();
  }

  function setPeriod(p: string) {
    selectedPeriod = p;
    handleSelect();
  }
</script>

<div class={style.root}>
  <Popover
    id={popoverId}
    role="dialog"
    ariaLabel={labelProps?.label ?? $t("input.time.selectTime")}
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
              <Icon name="Time" />
            </span>
            <span class={style.value}>{displayTime}</span>
          </div>
        </div>

        <div class="flex items-center gap-1">
          {#if value}
            <button
              type="button"
              class={style.clearButton}
              onclick={clearTime}
              aria-label={$t("input.time.clear")}
            >
              <Icon name="Cancel" />
            </button>
          {/if}
        </div>
      </button>
    {/snippet}

    {#snippet panel()}
      <div class={style.panel}>
        <div class={style.pickerGrid}>
          <!-- Hours -->
          <div class={style.pickerColumn}>
            {#each hours as h}
              <button
                type="button"
                class={`${style.pickerItem} ${selectedHour === h ? style.pickerItemSelected : ""}`}
                onclick={() => setHour(h)}
              >
                {h}
              </button>
            {/each}
          </div>

          <div class={style.columnDivider}></div>

          <!-- Minutes -->
          <div class={style.pickerColumn}>
            {#each minutes as m}
              <button
                type="button"
                class={`${style.pickerItem} ${selectedMinute === m ? style.pickerItemSelected : ""}`}
                onclick={() => setMinute(m)}
              >
                {m}
              </button>
            {/each}
          </div>

          <div class={style.columnDivider}></div>

          <!-- Period -->
          <div class={style.pickerColumn}>
            {#each periods as p}
              <button
                type="button"
                class={`${style.pickerItem} ${selectedPeriod === p ? style.pickerItemSelected : ""}`}
                onclick={() => setPeriod(p)}
              >
                {p}
              </button>
            {/each}
          </div>
        </div>
      </div>
    {/snippet}
  </Popover>
</div>

