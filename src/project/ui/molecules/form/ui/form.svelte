<script lang="ts">
  import Select from "../../../atoms/select/ui/Select.svelte";
  import Checkbox from "../../../atoms/checkbox/ui/Checkbox.svelte";
  import Radio from "../../../atoms/radio/ui/Radio.svelte";
  import Toggle from "../../../atoms/toggle/ui/Toggle.svelte";
  import Textarea from "../../../atoms/textarea/ui/Textarea.svelte";
  import InputLabel from "../../../atoms/input/children/label/ui/InputLabel.svelte";
  import InputAge from "../../../atoms/input/children/age/ui/InputAge.svelte";
  import InputDate from "../../../atoms/input/children/date/ui/InputDate.svelte";
  import InputEmail from "../../../atoms/input/children/email/ui/InputEmail.svelte";
  import InputNumber from "../../../atoms/input/children/number/ui/InputNumber.svelte";
  import InputPassword from "../../../atoms/input/children/password/ui/InputPassword.svelte";
  import InputPhone from "../../../atoms/input/children/phone/ui/InputPhone.svelte";
  import InputPhoneFlag from "../../../atoms/input/children/phone/varient/flag/ui/InputPhoneFlag.svelte";
  import InputText from "../../../atoms/input/children/text/ui/InputText.svelte";
  import InputUsername from "../../../atoms/input/children/username/ui/InputUsername.svelte";
  import InputUsernameOrEmail from "../../../atoms/input/children/username/varient/username-or-email/ui/InputUsernameOrEmail.svelte";
  import InputUsernameOrPhone from "../../../atoms/input/children/username/varient/username-or-phone/ui/InputUsernameOrPhone.svelte";
  import InputOtp from "../../../atoms/input/children/otp/ui/InputOtp.svelte";
  import InputTime from "../../../atoms/input/children/time/ui/InputTime.svelte";
  import type { FormProps } from "../data/types";

  let {
    style = "grid",
    className,
    fields = [],
    submitLabel,
    onSubmit,
  }: FormProps = $props();

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    const formData = new FormData(event.currentTarget as HTMLFormElement);
    onSubmit?.(Object.fromEntries(formData.entries()));
  }
</script>

<form
  class="w-full grid grid-cols-1 {style === 'grid' &&
    'md:grid-cols-2'} gap-4 {className}"
  onsubmit={handleSubmit}
>
  {#each fields as field, index (field.id ?? `${field.component ?? field.type ?? "field"}-${index}`)}
    {@const component = field.component ?? field.type}
    <div id={`i-${index}`}>
      {#if component === "select"}
        <Select {...(field as any)} />
      {:else if component === "checkbox"}
        <Checkbox {...(field as any)} />
      {:else if component === "radio"}
        <Radio {...(field as any)} />
      {:else if component === "toggle"}
        <Toggle {...(field as any)} />
      {:else if component === "textarea"}
        <Textarea {...(field as any)} />
      {:else if component === "input-label"}
        <InputLabel {...(field as any)} />
      {:else if component === "input-age"}
        <InputAge {...(field as any)} />
      {:else if component === "input-date"}
        <InputDate {...(field as any)} />
      {:else if component === "input-email"}
        <InputEmail {...(field as any)} />
      {:else if component === "input-number"}
        <InputNumber {...(field as any)} />
      {:else if component === "input-password"}
        <InputPassword {...(field as any)} />
      {:else if component === "input-phone"}
        <InputPhone {...(field as any)} />
      {:else if component === "input-phone-flag"}
        <InputPhoneFlag {...(field as any)} flag={field.flag ?? []} />
      {:else if component === "input-text"}
        <InputText {...(field as any)} />
      {:else if component === "input-username"}
        <InputUsername {...(field as any)} />
      {:else if component === "input-username-or-email"}
        <InputUsernameOrEmail {...(field as any)} />
      {:else if component === "input-username-or-phone"}
        <InputUsernameOrPhone {...(field as any)} />
      {:else if component === "input-otp"}
        <InputOtp {...(field as any)} />
      {:else if component === "input-time"}
        <InputTime {...(field as any)} />
      {/if}
    </div>
  {/each}
  {#if submitLabel}
    <div class="flex justify-end md:col-span-2">
      <button type="submit" class="rounded-md bg-blue-600 px-4 py-2 text-xs font-semibold text-white transition hover:bg-blue-500">{submitLabel}</button>
    </div>
  {/if}
</form>
