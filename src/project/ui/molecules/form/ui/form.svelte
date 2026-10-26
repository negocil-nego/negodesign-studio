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
  import InputPhoneFlag from "../../../atoms/input/children/phone/ui/InputPhoneFlag.svelte";
  import InputText from "../../../atoms/input/children/text/ui/InputText.svelte";
  import InputTime from "../../../atoms/input/children/time/ui/InputTime.svelte";
  import type { SelectProps } from "../../../atoms/select/data/types";
  import type { CheckboxProps } from "../../../atoms/checkbox/data/types";
  import type { RadioProps } from "../../../atoms/radio/data/types";
  import type { ToggleProps } from "../../../atoms/toggle/data/types";
  import type { TextareaProps } from "../../../atoms/textarea/data/types";
  import type { InputLabelProps } from "../../../atoms/input/children/label/data/types";
  import type { InputAgeProps } from "../../../atoms/input/children/age/data/types";
  import type { InputDateProps } from "../../../atoms/input/children/date/data/types";
  import type { InputEmailProps } from "../../../atoms/input/children/email/data/types";
  import type { InputNumberProps } from "../../../atoms/input/children/number/data/types";
  import type { InputPasswordProps } from "../../../atoms/input/children/password/data/types";
  import type {
    InputPhoneProps,
    InputPhoneFlagProps,
  } from "../../../atoms/input/children/phone/data/types";
  import type { InputTextProps } from "../../../atoms/input/children/text/data/types";
  import type { InputTimeProps } from "../../../atoms/input/children/time/data/types";
  import type { FormProps } from "../data/types";

  let { fields = [], onSubmit }: FormProps = $props();

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    onSubmit?.();
  }
</script>

<form
  class="w-full grid grid-cols-1 md:grid-cols-2 gap-4"
  onsubmit={handleSubmit}
>
  {#each fields as field, index (field.id ?? `${field.component ?? field.type ?? "field"}-${index}`)}
    <div id={`i-${index}`}>
      {@const component = field.component ?? field.type}
      {#if component === "select"}
        {@const props = field as SelectProps}
        <Select {...props} />
      {:else if component === "checkbox"}
        {@const props = field as CheckboxProps}
        <Checkbox {...props} />
      {:else if component === "radio"}
        {@const props = field as RadioProps}
        <Radio {...props} />
      {:else if component === "toggle"}
        {@const props = field as ToggleProps}
        <Toggle {...props} />
      {:else if component === "textarea"}
        {@const props = field as TextareaProps}
        <Textarea {...props} />
      {:else if component === "input-label"}
        {@const props = field as InputLabelProps}
        <InputLabel {...props} />
      {:else if component === "input-age"}
        {@const props = field as InputAgeProps}
        <InputAge {...props} />
      {:else if component === "input-date"}
        {@const props = field as InputDateProps}
        <InputDate {...props} />
      {:else if component === "input-email"}
        {@const props = field as InputEmailProps}
        <InputEmail {...props} />
      {:else if component === "input-number"}
        {@const props = field as InputNumberProps}
        <InputNumber {...props} />
      {:else if component === "input-password"}
        {@const props = field as InputPasswordProps}
        <InputPassword {...props} />
      {:else if component === "input-phone"}
        {@const props = field as InputPhoneProps}
        <InputPhone {...props} />
      {:else if component === "input-phone-flag"}
        {@const props = {
          ...field,
          flag: field.flag ?? [],
        } as InputPhoneFlagProps}
        <InputPhoneFlag {...props} />
      {:else if component === "input-text"}
        {@const props = field as InputTextProps}
        <InputText {...props} />
      {:else if component === "input-time"}
        {@const props = field as InputTimeProps}
        <InputTime {...props} />
      {/if}
    </div>
  {/each}
</form>
