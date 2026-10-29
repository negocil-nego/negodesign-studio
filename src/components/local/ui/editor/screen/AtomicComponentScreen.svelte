<script lang="ts">
  import Input from "../../../../../project/ui/atoms/input/core/ui/Input.svelte";
  import InputText from "../../../../../project/ui/atoms/input/children/text/ui/InputText.svelte";
  import InputUsername from "../../../../../project/ui/atoms/input/children/username/ui/InputUsername.svelte";
  import InputUsernameOrEmail from "../../../../../project/ui/atoms/input/children/username/varient/username-or-email/ui/InputUsernameOrEmail.svelte";
  import InputUsernameOrPhone from "../../../../../project/ui/atoms/input/children/username/varient/username-or-phone/ui/InputUsernameOrPhone.svelte";
  import InputOtp from "../../../../../project/ui/atoms/input/children/otp/ui/InputOtp.svelte";
  import InputEmail from "../../../../../project/ui/atoms/input/children/email/ui/InputEmail.svelte";
  import InputPhone from "../../../../../project/ui/atoms/input/children/phone/ui/InputPhone.svelte";
  import InputPhoneFlag from "../../../../../project/ui/atoms/input/children/phone/varient/flag/ui/InputPhoneFlag.svelte";
  import InputPassword from "../../../../../project/ui/atoms/input/children/password/ui/InputPassword.svelte";
  import InputDate from "../../../../../project/ui/atoms/input/children/date/ui/InputDate.svelte";
  import InputTime from "../../../../../project/ui/atoms/input/children/time/ui/InputTime.svelte";
  import InputNumber from "../../../../../project/ui/atoms/input/children/number/ui/InputNumber.svelte";
  import InputAge from "../../../../../project/ui/atoms/input/children/age/ui/InputAge.svelte";
  import InputLabel from "../../../../../project/ui/atoms/input/children/label/ui/InputLabel.svelte";
  import Select from "../../../../../project/ui/atoms/select/ui/Select.svelte";
  import Textarea from "../../../../../project/ui/atoms/textarea/ui/Textarea.svelte";
  import Toggle from "../../../../../project/ui/atoms/toggle/ui/Toggle.svelte";
  import Radio from "../../../../../project/ui/atoms/radio/ui/Radio.svelte";
  import Checkbox from "../../../../../project/ui/atoms/checkbox/ui/Checkbox.svelte";
  import Label from "../../../../../project/ui/atoms/label/ui/label.svelte";
  import { FlagNamed } from "../../../../../hooks/flag.svelte";
  import type { ComponentDefinition } from "../../../data/components";
  import { activeProject } from "../../../data/projects";
  import { t } from "../../../../../project/data/i18n";

  // Terminals
  import InputTextTerminal from "../terminal/input/InputTextTerminal.svelte";
  import InputUsernameTerminal from "../terminal/input/InputUsernameTerminal.svelte";
  import InputUsernameOrEmailTerminal from "../terminal/input/InputUsernameOrEmailTerminal.svelte";
  import InputUsernameOrPhoneTerminal from "../terminal/input/InputUsernameOrPhoneTerminal.svelte";
  import InputOtpTerminal from "../terminal/input/InputOtpTerminal.svelte";
  import InputEmailTerminal from "../terminal/input/InputEmailTerminal.svelte";
  import InputPhoneTerminal from "../terminal/input/InputPhoneTerminal.svelte";
  import InputPhoneFlagTerminal from "../terminal/input/InputPhoneFlagTerminal.svelte";
  import InputPasswordTerminal from "../terminal/input/InputPasswordTerminal.svelte";
  import InputDateTerminal from "../terminal/input/InputDateTerminal.svelte";
  import InputTimeTerminal from "../terminal/input/InputTimeTerminal.svelte";
  import InputNumberTerminal from "../terminal/input/InputNumberTerminal.svelte";
  import InputAgeTerminal from "../terminal/input/InputAgeTerminal.svelte";
  import CheckboxTerminal from "../terminal/control/CheckboxTerminal.svelte";
  import RadioTerminal from "../terminal/control/RadioTerminal.svelte";
  import ToggleTerminal from "../terminal/control/ToggleTerminal.svelte";
  import SelectTerminal from "../terminal/control/SelectTerminal.svelte";
  import TextareaTerminal from "../terminal/control/TextareaTerminal.svelte";
  import LabelTerminal from "../terminal/LabelTerminal.svelte";

  let {
    components,
    copyingId,
    onCopy,
    viewMode = "gallery",
  }: {
    components: ComponentDefinition[];
    copyingId: string | null;
    onCopy: (c: ComponentDefinition) => void;
    viewMode?: "gallery" | "terminal";
  } = $props();

  const selectOptions = [
    { value: "ao", label: "Angola" },
    { value: "pt", label: "Portugal" },
    { value: "br", label: "Brasil" },
  ];

  const radioOptions = [
    { value: "svelte", label: "Svelte" },
    { value: "react", label: "React" },
  ];
</script>

{#if viewMode === "terminal"}
  <div class="space-y-8">
    {#each components as component (component.id)}
      <section id={component.id} class="space-y-4">
        {#if component.id === "input-text"}
          <InputTextTerminal />
        {:else if component.id === "input-username"}
          <InputUsernameTerminal />
        {:else if component.id === "input-username-or-email"}
          <InputUsernameOrEmailTerminal />
        {:else if component.id === "input-username-or-phone"}
          <InputUsernameOrPhoneTerminal />
        {:else if component.id === "input-otp"}
          <InputOtpTerminal />
        {:else if component.id === "input-email"}
          <InputEmailTerminal />
        {:else if component.id === "input-phone"}
          <InputPhoneTerminal />
        {:else if component.id === "input-phone-flag"}
          <InputPhoneFlagTerminal />
        {:else if component.id === "input-password"}
          <InputPasswordTerminal />
        {:else if component.id === "input-date"}
          <InputDateTerminal />
        {:else if component.id === "input-time"}
          <InputTimeTerminal />
        {:else if component.id === "input-number"}
          <InputNumberTerminal />
        {:else if component.id === "input-age"}
          <InputAgeTerminal />
        {:else if component.id === "checkbox"}
          <CheckboxTerminal />
        {:else if component.id === "radio"}
          <RadioTerminal />
        {:else if component.id === "toggle"}
          <ToggleTerminal />
        {:else if component.id === "select"}
          <SelectTerminal />
        {:else if component.id === "textarea"}
          <TextareaTerminal />
        {:else if component.id === "label"}
          <LabelTerminal />
        {/if}
      </section>
    {/each}
  </div>
{:else}
  <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-3">
    {#each components as component (component.id)}
      <article
        class="overflow-hidden rounded-xl border border-blender-border bg-blender-card shadow-sm transition-shadow hover:shadow-md"
      >
        <div
          class="flex min-h-36 items-center justify-center bg-blender-base p-5"
        >
          {#if component.id === "input"}
            <div class="w-full max-w-xs">
              <Input type="text" placeholder={$t("input.text.placeholder")} />
            </div>
          {:else if component.id === "input-text"}
            <div class="w-full max-w-xs">
              <InputText
                inputProps={{
                  id: `gallery-${component.id}`,
                  type: "text",
                  placeholder: $t("input.text.placeholder"),
                }}
              />
            </div>
          {:else if component.id === "input-username"}
            <div class="w-full max-w-xs">
              <InputUsername
                labelProps={{ label: "Utilizador", for: "gallery-input-username" }}
                inputProps={{
                  id: "gallery-input-username",
                  type: "text",
                  placeholder: "Nome, email ou +244 900 000 000",
                }}
              />
            </div>
          {:else if component.id === "input-username-or-email"}
            <div class="w-full max-w-xs">
              <InputUsernameOrEmail
                labelProps={{ label: "Utilizador ou email", for: "gallery-input-username-email" }}
                inputProps={{ id: "gallery-input-username-email", type: "text", placeholder: "utilizador ou email@exemplo.com" }}
              />
            </div>
          {:else if component.id === "input-username-or-phone"}
            <div class="w-full max-w-xs">
              <InputUsernameOrPhone
                labelProps={{ label: "Utilizador ou telefone", for: "gallery-input-username-phone" }}
                inputProps={{ id: "gallery-input-username-phone", type: "text", placeholder: "utilizador ou +244 900 000 000" }}
              />
            </div>
          {:else if component.id === "input-otp"}
            <div class="w-full max-w-xs">
              <InputOtp
                id="gallery-input-otp"
                name="verification-code"
                value="456789"
                length={6}
              />
            </div>
          {:else if component.id === "input-email"}
            <div class="w-full max-w-xs">
              <InputEmail
                inputProps={{
                  id: `gallery-${component.id}`,
                  type: "email",
                  placeholder: $t("input.email.placeholder"),
                }}
              />
            </div>
          {:else if component.id === "input-phone"}
            <div class="w-full max-w-xs">
              <InputPhone
                inputProps={{
                  id: `gallery-${component.id}`,
                  type: "tel",
                  placeholder: $t("input.phone.placeholder"),
                }}
              />
            </div>
          {:else if component.id === "input-phone-flag"}
            <div class="w-full max-w-xs">
              <InputPhoneFlag
                flag={[FlagNamed.Ao, FlagNamed.Ng]}
                inputProps={{
                  id: `gallery-${component.id}`,
                  type: "tel",
                  placeholder: "9XX XXX XXX",
                }}
              />
            </div>
          {:else if component.id === "input-password"}
            <div class="w-full max-w-xs">
              <InputPassword
                inputProps={{
                  id: `gallery-${component.id}`,
                  type: "password",
                  placeholder: $t("input.password.placeholder"),
                }}
              />
            </div>
          {:else if component.id === "input-date"}
            <div class="w-full max-w-xs">
              <InputDate
                inputProps={{ id: `gallery-${component.id}`, type: "date" }}
              />
            </div>
          {:else if component.id === "input-time"}
            <div class="w-full max-w-xs">
              <InputTime
                inputProps={{ id: `gallery-${component.id}`, type: "time" }}
              />
            </div>
          {:else if component.id === "input-number"}
            <div class="w-full max-w-xs">
              <InputNumber
                inputProps={{
                  id: `gallery-${component.id}`,
                  type: "number",
                  placeholder: $t("input.amount.placeholder"),
                }}
              />
            </div>
          {:else if component.id === "input-age"}
            <div class="w-full max-w-xs">
              <InputAge
                labelProps={{ label: $t("label.age"), for: `gallery-${component.id}` }}
                inputProps={{
                  id: `gallery-${component.id}`,
                  type: "number",
                  placeholder: $t("input.age.placeholder"),
                }}
              />
            </div>
          {:else if component.id === "input-label"}
            <div class="w-full max-w-xs">
              <InputLabel
                labelProps={{ label: $t("label.name"), for: `gallery-${component.id}` }}
                inputProps={{
                  id: `gallery-${component.id}`,
                  type: "text",
                  placeholder: $t("input.name.placeholder"),
                }}
              />
            </div>
          {:else if component.id === "label"}
            <div class="w-full max-w-xs space-y-2">
              <Label label={$t("label.name")} for={`gallery-${component.id}`} />
              <input
                id={`gallery-${component.id}`}
                type="text"
                placeholder={$t("input.name.placeholder")}
                class="w-full rounded-md border border-blender-border bg-blender-card px-3 py-2 text-sm text-zinc-200 placeholder:text-zinc-500"
              />
            </div>
          {:else if component.id === "select"}
            <div class="w-full max-w-xs">
              <Select
                id={`gallery-${component.id}`}
                options={selectOptions}
                placeholder={$t("input.location.placeholder")}
              />
            </div>
          {:else if component.id === "textarea"}
            <div class="w-full max-w-xs">
              <Textarea
                id={`gallery-${component.id}`}
                placeholder={$t("input.message.placeholder")}
                rows={3}
              />
            </div>
          {:else if component.id === "toggle"}
            <div class="flex flex-col gap-3">
              <Toggle id={`gallery-${component.id}-off`} label="Desactivado" />
              <Toggle
                id={`gallery-${component.id}-on`}
                checked={true}
                label="Activado"
              />
            </div>
          {:else if component.id === "radio"}
            <Radio
              name={`gallery-${component.id}`}
              options={radioOptions}
              value="svelte"
            />
          {:else if component.id === "checkbox"}
            <div class="flex flex-col gap-3">
              <Checkbox
                id={`gallery-${component.id}-off`}
                label="Não seleccionado"
              />
              <Checkbox
                id={`gallery-${component.id}-on`}
                checked={true}
                label="Seleccionado"
              />
            </div>
          {/if}
        </div>

        <div class="space-y-1.5 p-4">
          <div class="flex items-center justify-between gap-3">
            <h3 class="truncate text-sm font-semibold text-zinc-100">
              {#if component.href}
                <a class="hover:text-blue-500" href={component.href}
                  >{component.label ?? component.name}</a
                >
              {:else}
                {component.label ?? component.name}
              {/if}
            </h3>
            <span
              class="shrink-0 text-[10px] font-medium uppercase tracking-wide text-zinc-400"
              >{component.group}</span
            >
          </div>
          <p class="text-xs leading-5 text-zinc-400">{component.description}</p>
          {#if component.href || component.dependence}
            <div class="flex flex-wrap gap-2 pt-2">
              {#if component.href}
                <a
                  href={component.href}
                  class="rounded border border-blender-border px-2.5 py-1.5 text-[11px] font-medium text-zinc-300 transition-colors hover:bg-blender-hover hover:text-zinc-100"
                  >Ver terminal</a
                >
              {/if}
              {#if component.dependence}
                <button
                  type="button"
                  class="rounded bg-blue-600 px-2.5 py-1.5 text-[11px] font-medium text-white transition-colors hover:bg-blue-500 disabled:cursor-not-allowed disabled:opacity-40"
                  onclick={() => onCopy(component)}
                  disabled={!$activeProject ||
                    $activeProject.language === "Unknown" ||
                    copyingId === component.id}
                >
                  {copyingId === component.id
                    ? "A copiar…"
                    : "Copiar para projecto"}
                </button>
              {/if}
            </div>
          {/if}
        </div>
      </article>
    {/each}
  </div>
{/if}
