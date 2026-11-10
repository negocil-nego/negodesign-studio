export function normalizeOtpDigits(value: string | undefined, count: number): string[] {
  const clean = String(value ?? "").replace(/\D/g, "").slice(0, count);
  return Array.from({ length: count }, (_, i) => clean[i] ?? "");
}

export function applyOtpDigit(
  digits: string[],
  index: number,
  rawInput: string,
): { nextDigits: string[]; focusIndex: number | null } {
  const digit = rawInput.replace(/\D/g, "").slice(-1);
  const nextDigits = [...digits];
  nextDigits[index] = digit;
  const focusIndex = digit && index < digits.length - 1 ? index + 1 : null;
  return { nextDigits, focusIndex };
}

export function applyOtpPaste(
  digits: string[],
  index: number,
  rawPaste: string,
): { nextDigits: string[]; focusIndex: number } | null {
  const pasted = rawPaste.replace(/\D/g, "");
  if (!pasted) return null;
  const nextDigits = [...digits];
  const chars = pasted.slice(0, digits.length - index).split("");
  chars.forEach((d, offset) => {
    nextDigits[index + offset] = d;
  });
  const focusIndex = Math.min(index + chars.length, digits.length - 1);
  return { nextDigits, focusIndex };
}

export function resolveOtpKeyFocus(
  key: string,
  index: number,
  length: number,
  currentDigit: string,
): number | null {
  if (key === "ArrowLeft" && index > 0) return index - 1;
  if (key === "ArrowRight" && index < length - 1) return index + 1;
  if (key === "Backspace" && !currentDigit && index > 0) return index - 1;
  return null;
}
