<script lang="ts">
  import type { InputOtpProps } from "../data/types";
  import { StyleComponent } from "../../../../../../data/style.component";

  let {
    id,
    name,
    value = $bindable(""),
    length = 6,
    separatorEvery = 3,
    labelProps,
    disabled = false,
    required = false,
    onValueChange,
  }: InputOtpProps = $props();

  const digitCount = Math.max(1, Math.min(12, length));
  const separatorPosition = Math.max(0, separatorEvery);
  let digits = $state<string[]>(Array(digitCount).fill(""));
  let inputs: HTMLInputElement[] = [];

  $effect(() => {
    const nextValue = String(value ?? "").replace(/\D/g, "").slice(0, digitCount);
    digits = Array.from({ length: digitCount }, (_, index) => nextValue[index] ?? "");
  });

  function updateValue(nextDigits: string[]) {
    digits = nextDigits;
    value = nextDigits.join("");
    onValueChange?.(value);
  }

  function handleInput(index: number, event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const digit = input.value.replace(/\D/g, "").slice(-1);
    const next = [...digits];
    next[index] = digit;
    updateValue(next);

    if (digit && index < digitCount - 1) inputs[index + 1]?.focus();
  }

  function handleKeydown(index: number, event: KeyboardEvent) {
    if (event.key === "ArrowLeft" && index > 0) {
      event.preventDefault();
      inputs[index - 1]?.focus();
    } else if (event.key === "ArrowRight" && index < digitCount - 1) {
      event.preventDefault();
      inputs[index + 1]?.focus();
    } else if (event.key === "Backspace" && !digits[index] && index > 0) {
      inputs[index - 1]?.focus();
    }
  }

  function handlePaste(index: number, event: ClipboardEvent) {
    const pasted = event.clipboardData?.getData("text").replace(/\D/g, "");
    if (!pasted) return;

    event.preventDefault();
    const next = [...digits];
    const characters = pasted.slice(0, digitCount - index).split("");
    characters.forEach((digit, offset) => (next[index + offset] = digit));
    updateValue(next);
    inputs[Math.min(index + characters.length, digitCount - 1)]?.focus();
  }
</script>

<div class={StyleComponent.input.otp.root}>
  {#if labelProps}
    <label for={labelProps.for} class={StyleComponent.input.otp.label} data-name="label">{labelProps.label}</label>
  {/if}

  <div
    class={StyleComponent.input.otp.group}
    role="group"
    aria-label={labelProps?.label ?? "Código de verificação"}
  >
    {#each digits as digit, index (index)}
      {#if separatorPosition > 0 && index > 0 && index % separatorPosition === 0}
        <span class={StyleComponent.input.otp.separator} aria-hidden="true">•</span>
      {/if}
      <input
        bind:this={inputs[index]}
        id={index === 0 ? (labelProps?.for ?? id) : `${id ?? labelProps?.for ?? "otp"}-${index}`}
        type="text"
        inputmode="numeric"
        pattern="[0-9]*"
        maxlength="1"
        autocomplete={index === 0 ? "one-time-code" : "off"}
        value={digit}
        {disabled}
        {required}
        aria-label={`Dígito ${index + 1} de ${digitCount}`}
        class={StyleComponent.input.otp.slot}
        oninput={(event) => handleInput(index, event)}
        onkeydown={(event) => handleKeydown(index, event)}
        onpaste={(event) => handlePaste(index, event)}
      />
    {/each}
  </div>
  {#if name}<input type="hidden" {name} {value} />{/if}
</div>
