<script lang="ts">
  import Input from "../../../core/ui/Input.svelte";
  import type { InputUsernameProps } from "../data/types";
  import { getUsernameValidationMessage, isValidUsername } from "../data/validation";
  import { StyleComponent } from "../../../../../../data/style.component";

  let {
    labelProps,
    inputProps,
    validationMode = "any",
    errorMessage,
    onValueChange,
    onValidityChange,
  }: InputUsernameProps = $props();

  let value = $state(String(inputProps.value ?? ""));
  let touched = $state(false);

  const isValid = $derived(
    value.trim() === "" ? !inputProps.required : isValidUsername(value, validationMode),
  );
  const isInvalid = $derived(touched && !isValid);
  const resolvedErrorMessage = $derived(errorMessage ?? getUsernameValidationMessage(validationMode));
  const errorId = $derived(`${inputProps.id ?? labelProps?.for ?? "input-username"}-error`);

  function handleInput(event: Event) {
    value = (event.currentTarget as HTMLInputElement).value;
    touched = true;
    onValueChange?.(value);
    onValidityChange?.(value.trim() ? isValidUsername(value, validationMode) : !inputProps.required);
  }

  function handleBlur() {
    touched = true;
    onValidityChange?.(isValid);
  }
</script>

<div class="space-y-1.5">
  {#if labelProps}
    <label
      for={labelProps.for}
      class={StyleComponent.input.text.label}
      data-name="label">{labelProps.label}</label
    >
  {/if}
  <Input
    {...inputProps}
    type="text"
    value={value}
    isInvalid={isInvalid}
    autocomplete="username"
    aria-describedby={isInvalid ? errorId : undefined}
    oninput={handleInput}
    onblur={handleBlur}
  />
  {#if isInvalid}
    <p
      id={errorId}
      class="text-xs text-destructive"
      role="alert">{resolvedErrorMessage}</p
    >
  {/if}
</div>
