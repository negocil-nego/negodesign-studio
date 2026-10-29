const patterns = {
  username: /^[\p{L}\p{N}][\p{L}\p{N}._-]{2,29}$/u,
  email: /^[^\s@]+@[^\s@]+\.[^\s@]{2,}$/,
  internationalPhone: /^\+?[1-9]\d{6,14}$/,
};

export function isValidUsername(
  value: string,
  mode: "any" | "username" | "email" | "phone" | "username-or-email" | "username-or-phone" = "any",
): boolean {
  const candidate = value.trim();
  if (!candidate) return false;

  const matchesUsername = patterns.username.test(candidate);
  const matchesEmail = patterns.email.test(candidate);
  const normalizedPhone = candidate.replace(/[\s().-]/g, "");
  const matchesPhone = patterns.internationalPhone.test(normalizedPhone);

  switch (mode) {
    case "username": return matchesUsername;
    case "email": return matchesEmail;
    case "phone": return matchesPhone;
    case "username-or-email": return matchesUsername || matchesEmail;
    case "username-or-phone": return matchesUsername || matchesPhone;
    default: return matchesUsername || matchesEmail || matchesPhone;
  }
}

export function getUsernameValidationMessage(
  mode: "any" | "username" | "email" | "phone" | "username-or-email" | "username-or-phone",
): string {
  switch (mode) {
    case "username": return "Introduza um nome de utilizador válido.";
    case "email": return "Introduza um email válido.";
    case "phone": return "Introduza um telefone válido com o código do país.";
    case "username-or-email": return "Introduza um nome de utilizador ou email válido.";
    case "username-or-phone": return "Introduza um nome de utilizador ou telefone válido.";
    default: return "Introduza um nome de utilizador, email ou telefone válido.";
  }
}
