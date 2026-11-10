<script lang="ts">
  import type { InputOtpProps } from "../data/types";
  import { StyleComponent } from "../../../../../../data/style.component";
  import {
    applyOtpDigit,
    applyOtpPaste,
    normalizeOtpDigits,
    resolveOtpKeyFocus,
  } from "../../../../../../hooks/otp-input.hook";

  let {
    id,
    name,
    value = $bindable(""),
    length = 6,
    separatorEvery = 3,
    labelProps,
    position = "start",
    disabled = false,
    required = false,
    onValueChange,
  }: InputOtpProps = $props();

  const digitCount = $derived(Math.max(1, Math.min(12, length)));
  const separatorPosition = $derived(Math.max(0, separatorEvery));
  let digits = $state<string[]>([]);
  let inputs: HTMLInputElement[] = [];

  $effect(() => {
    digits = normalizeOtpDigits(value, digitCount);
  });

  function updateValue(nextDigits: string[]) {
    digits = nextDigits;
    value = nextDigits.join("");
    onValueChange?.(value);
  }

  function handleInput(index: number, event: Event) {
    const res = applyOtpDigit(digits, index, (event.currentTarget as HTMLInputElement).value);
    updateValue(res.nextDigits);
    if (res.focusIndex !== null) inputs[res.focusIndex]?.focus();
  }

  function handleKeydown(index: number, event: KeyboardEvent) {
    const nextIdx = resolveOtpKeyFocus(event.key, index, digitCount, digits[index]);
    if (nextIdx !== null) {
      if (event.key !== "Backspace") event.preventDefault();
      inputs[nextIdx]?.focus();
    }
  }

  function handlePaste(index: number, event: ClipboardEvent) {
    const res = applyOtpPaste(digits, index, event.clipboardData?.getData("text") ?? "");
    if (!res) return;
    event.preventDefault();
    updateValue(res.nextDigits);
    inputs[res.focusIndex]?.focus();
  }
</script>

<div class="{StyleComponent.input.otp.root} {position === 'center' ? 'mx-auto w-fit' : ''}">
  {#if labelProps}
    <label for={labelProps.for} class="{StyleComponent.input.otp.label} {StyleComponent.input.otp.labelPosition[position]} block"
      data-name="label">{labelProps.label}</label>
  {/if}
  <div class={StyleComponent.input.otp.group} role="group" aria-label={labelProps?.label ?? "Verification code"}>
    {#each digits as digit, index (index)}
      {#if separatorPosition > 0 && index > 0 && index % separatorPosition === 0}
        <span class={StyleComponent.input.otp.separator} aria-hidden="true">•</span>
      {/if}
      <input bind:this={inputs[index]}
        id={index === 0 ? (labelProps?.for ?? id) : `${id ?? labelProps?.for ?? "otp"}-${index}`}
        type="text" inputmode="numeric" pattern="[0-9]*" maxlength="1"
        autocomplete={index === 0 ? "one-time-code" : "off"} value={digit} {disabled} {required}
        aria-label={`Digit ${index + 1} of ${digitCount}`} class={StyleComponent.input.otp.slot}
        oninput={(e) => handleInput(index, e)} onkeydown={(e) => handleKeydown(index, e)} onpaste={(e) => handlePaste(index, e)} />
    {/each}
  </div>
  {#if name}<input type="hidden" {name} {value} />{/if}
</div>
