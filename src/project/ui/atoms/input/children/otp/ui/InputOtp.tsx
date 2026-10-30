import { useEffect, useRef, useState } from "react";
import type { InputOtpProps } from "../data/types";
import { StyleComponent } from "../../../../../../data/style.component";

export default function InputOtp({
  id,
  name,
  value = "",
  length = 6,
  separatorEvery = 3,
  labelProps,
  label,
  disabled = false,
  required = false,
  position = "start",
  onValueChange,
}: InputOtpProps) {
  const count = Math.max(1, Math.min(12, length));
  const [code, setCode] = useState(value);
  const refs = useRef<Array<HTMLInputElement | null>>([]);
  const digits = Array.from({ length: count }, (_, index) => code[index] ?? "");

  useEffect(() => setCode(value), [value]);

  function update(index: number, digit: string) {
    const next = [...digits];
    next[index] = digit;
    const nextCode = next.join("");
    setCode(nextCode);
    onValueChange?.(nextCode);
  }

  function paste(index: number, event: React.ClipboardEvent<HTMLInputElement>) {
    const pasted = event.clipboardData.getData("text").replace(/\D/g, "");
    if (!pasted) return;
    event.preventDefault();
    const next = [...digits];
    pasted.slice(0, count - index).split("").forEach((digit, offset) => (next[index + offset] = digit));
    const nextCode = next.join("");
    setCode(nextCode);
    onValueChange?.(nextCode);
    refs.current[Math.min(index + pasted.length, count - 1)]?.focus();
  }

  return (
    <div className={`${StyleComponent.input.otp.root} ${position === "center" ? "mx-auto w-fit" : ""}`}>
      {labelProps && <label htmlFor={labelProps.for} className={`${StyleComponent.input.otp.label} ${StyleComponent.input.otp.labelPosition[position]} block`} data-name="label">{labelProps.label}</label>}
      <div className={StyleComponent.input.otp.group} role="group" aria-label={labelProps?.label ?? label ?? "Código de verificação"}>
        {digits.map((digit, index) => (
          <span key={index} className="contents">
            {separatorEvery > 0 && index > 0 && index % separatorEvery === 0 && <span className={StyleComponent.input.otp.separator} aria-hidden="true">•</span>}
            <input
              ref={(element) => { refs.current[index] = element; }}
              id={index === 0 ? labelProps?.for ?? id : `${id ?? labelProps?.for ?? "otp"}-${index}`}
              type="text"
              inputMode="numeric"
              pattern="[0-9]*"
              maxLength={1}
              autoComplete={index === 0 ? "one-time-code" : "off"}
              value={digit}
              disabled={disabled}
              required={required}
              aria-label={`Dígito ${index + 1} de ${count}`}
              className={StyleComponent.input.otp.slot}
              onChange={(event) => {
                const inputDigit = event.target.value.replace(/\D/g, "").slice(-1);
                update(index, inputDigit);
                if (inputDigit && index < count - 1) refs.current[index + 1]?.focus();
              }}
              onKeyDown={(event) => {
                if (event.key === "ArrowLeft" && index > 0) {
                  event.preventDefault();
                  refs.current[index - 1]?.focus();
                } else if (event.key === "ArrowRight" && index < count - 1) {
                  event.preventDefault();
                  refs.current[index + 1]?.focus();
                } else if (event.key === "Backspace" && !digit && index > 0) {
                  refs.current[index - 1]?.focus();
                }
              }}
              onPaste={(event) => paste(index, event)}
            />
          </span>
        ))}
      </div>
      {name && <input type="hidden" name={name} value={code} />}
    </div>
  );
}
