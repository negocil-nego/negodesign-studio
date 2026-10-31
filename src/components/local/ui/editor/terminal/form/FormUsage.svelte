<script lang="ts">
  import Form from "../../../../../../project/ui/molecules/form/ui/form.svelte";
  import { FlagNamed } from "../../../../../../hooks/flag.svelte";
  import type { FormIt } from "../../../../../../project/ui/molecules/form/data/types";

  let layout = $state<"grid" | "inline">("grid");
  let submitted = $state(false);

  const fields: FormIt[] = [
    {
      component: "input-text",
      id: "full-name",
      name: "fullName",
      labelProps: { label: "Nome completo", for: "full-name" },
      inputProps: { id: "full-name", name: "fullName", type: "text", placeholder: "O seu nome", required: true },
      autoComplete: "name",
      minLength: 2,
      maxLength: 80,
    },
    {
      component: "input-email",
      id: "email",
      labelProps: { label: "Email", for: "email" },
      inputProps: { id: "email", name: "email", type: "email", placeholder: "nome@exemplo.com", required: true },
      autoComplete: "email",
    },
    {
      component: "input-password",
      id: "password",
      labelProps: { label: "Palavra-passe", for: "password" },
      inputProps: { id: "password", name: "password", type: "password", placeholder: "Mínimo de 8 caracteres", required: true },
      minLength: 8,
      autoComplete: "new-password",
    },
    {
      component: "input-number",
      id: "quantity",
      labelProps: { label: "Quantidade", for: "quantity" },
      inputProps: { id: "quantity", name: "quantity", type: "number", value: "2" },
      min: 1,
      max: 99,
      step: 1,
      inputMode: "numeric",
      class: "form-demo-number",
      className: "w-full",
      style: "accent-color: #2563eb",
      form: "profile-form",
    },
    {
      component: "input-age",
      id: "age",
      labelProps: { label: "Idade", for: "age" },
      inputProps: { id: "age", name: "age", type: "number", placeholder: "18" },
      min: 18,
      max: 120,
    },
    {
      component: "input-date",
      id: "date",
      labelProps: { label: "Data", for: "date" },
      inputProps: { id: "date", name: "date", type: "date" },
      required: true,
    },
    {
      component: "input-time",
      id: "time",
      labelProps: { label: "Hora", for: "time" },
      inputProps: { id: "time", name: "time", type: "time" },
    },
    {
      component: "input-phone",
      id: "phone",
      labelProps: { label: "Telefone", for: "phone" },
      inputProps: { id: "phone", name: "phone", type: "tel", placeholder: "+244 900 000 000" },
      autoComplete: "tel",
    },
    {
      component: "input-phone-flag",
      id: "phone-country",
      labelProps: { label: "Telefone com país", for: "phone-country" },
      inputProps: { id: "phone-country", name: "phoneCountry", type: "tel", placeholder: "900 000 000" },
      flag: [FlagNamed.Ao, FlagNamed.Pt],
      locale: "pt",
      onCountrySelect: (country) => console.info("Indicativo seleccionado:", country),
    },
    {
      component: "input-username",
      id: "username",
      labelProps: { label: "Utilizador", for: "username" },
      inputProps: { id: "username", name: "username", type: "text", placeholder: "nome.utilizador" },
      validationMode: "username",
      errorMessage: "Introduza um nome de utilizador válido.",
      onValidityChange: (valid) => console.info("Utilizador válido:", valid),
    },
    {
      component: "input-username-or-email",
      id: "login-email",
      labelProps: { label: "Utilizador ou email", for: "login-email" },
      inputProps: { id: "login-email", name: "loginEmail", type: "text", placeholder: "utilizador ou email" },
    },
    {
      component: "input-username-or-phone",
      id: "login-phone",
      labelProps: { label: "Utilizador ou telefone", for: "login-phone" },
      inputProps: { id: "login-phone", name: "loginPhone", type: "text", placeholder: "utilizador ou +244…" },
    },
    {
      component: "input-otp",
      id: "otp",
      name: "otp",
      labelProps: { label: "Código OTP", for: "otp" },
      length: 6,
      separatorEvery: 3,
      position: "center",
      onValueChange: (value) => console.info("Código:", value),
    },
    {
      component: "input-label",
      id: "website",
      labelProps: { label: "Website", for: "website" },
      inputProps: { id: "website", name: "website", type: "url", placeholder: "https://exemplo.com" },
      autoFocus: false,
    },
    {
      component: "textarea",
      id: "message",
      name: "message",
      labelProps: { label: "Mensagem", for: "message" },
      placeholder: "Escreva uma mensagem…",
      rows: 3,
      cols: 30,
      maxlength: 240,
      required: true,
    },
    {
      component: "select",
      id: "country",
      name: "country",
      label: "País",
      placeholder: "Seleccione um país",
      options: [
        { value: "ao", label: "Angola" },
        { value: "pt", label: "Portugal" },
        { value: "br", label: "Brasil", disabled: true },
      ],
      onOptionSelect: (option) => console.info("País:", option.value),
    },
    {
      component: "radio",
      id: "plan",
      name: "plan",
      label: "Plano",
      value: "starter",
      options: [
        { value: "starter", label: "Inicial" },
        { value: "team", label: "Equipa" },
        { value: "enterprise", label: "Enterprise", disabled: true },
      ],
      onValueChange: (value) => console.info("Plano:", value),
    },
    {
      component: "checkbox",
      id: "terms",
      name: "terms",
      label: "Aceito os termos de utilização",
      checked: true,
      required: true,
      onCheckedChange: (checked) => console.info("Termos:", checked),
    },
    {
      component: "toggle",
      id: "newsletter",
      name: "newsletter",
      label: "Receber novidades por email",
      checked: false,
      onCheckedChange: (checked) => console.info("Novidades:", checked),
    },
    {
      component: "input-text",
      id: "readonly-code",
      name: "reference",
      value: "ND-2026",
      labelProps: { label: "Referência (só leitura)", for: "readonly-code" },
      inputProps: { id: "readonly-code", name: "reference", type: "text", value: "ND-2026" },
      readonly: true,
      readOnly: true,
      title: "Código de referência",
      spellCheck: false,
      tabIndex: 0,
      pattern: "[A-Z]{2}-[0-9]{4}",
      multiple: false,
      size: 20,
      "aria-describedby": "reference-help",
      "data-field": "reference",
    },
  ];

  function submitForm() {
    submitted = true;
  }
</script>

<div class="w-full space-y-4">
  <div class="flex flex-wrap items-center justify-between gap-3">
    <p class="text-xs text-zinc-400">Demonstração dos controlos e propriedades disponíveis.</p>
    <label class="flex items-center gap-2 text-xs text-zinc-300">
      Layout
      <select bind:value={layout} class="rounded border border-blender-border bg-blender-card px-2 py-1.5">
        <option value="grid">Grelha</option>
        <option value="inline">Uma coluna</option>
      </select>
    </label>
  </div>

  <Form
    style={layout}
    className="rounded-lg border border-blender-border bg-blender-card p-4"
    {fields}
    onSubmit={submitForm}
  />
  <div class="flex items-center justify-between gap-3">
    <span class="text-xs text-zinc-400" aria-live="polite">
      {submitted ? "Formulário submetido (demonstração)." : "Os campos assinalados são obrigatórios."}
    </span>
    <button
      class="rounded-md bg-blue-600 px-3 py-2 text-xs font-semibold text-white hover:bg-blue-500"
      type="button"
      onclick={submitForm}
    >Submeter demonstração</button>
  </div>
</div>
