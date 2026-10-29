import { useState } from "react";
import type { InputUsernameProps } from "../data/types";
import Input from "../../../core/ui/Input";
import { getUsernameValidationMessage, isValidUsername } from "../data/validation";
import { StyleComponent } from "../../../../../../data/style.component";

export default function InputUsername({
  labelProps,
  inputProps,
  validationMode = "any",
  errorMessage,
  onValueChange,
  onValidityChange,
}: InputUsernameProps) {
  const [value, setValue] = useState(String(inputProps.value ?? ""));
  const [touched, setTouched] = useState(false);
  const valid = value.trim() ? isValidUsername(value, validationMode) : !inputProps.required;
  const invalid = touched && !valid;
  const resolvedErrorMessage = errorMessage ?? getUsernameValidationMessage(validationMode);
  const errorId = `${inputProps.id ?? labelProps?.for ?? "input-username"}-error`;

  function update(nextValue: string) {
    setValue(nextValue);
    setTouched(true);
    onValueChange?.(nextValue);
    onValidityChange?.(nextValue.trim() ? isValidUsername(nextValue, validationMode) : !inputProps.required);
  }

  return (
    <div className="space-y-1.5">
      {labelProps && (
        <label htmlFor={labelProps.for} className={StyleComponent.input.text.label}>
          {labelProps.label}
        </label>
      )}
      <Input
        {...inputProps}
        type="text"
        value={value}
        isInvalid={invalid}
        autoComplete="username"
        aria-describedby={invalid ? errorId : undefined}
        onChange={(event) => update(event.currentTarget.value)}
        onBlur={() => {
          setTouched(true);
          onValidityChange?.(valid);
        }}
      />
      {invalid && <p id={errorId} className="text-xs text-destructive" role="alert">{resolvedErrorMessage}</p>}
    </div>
  );
}
