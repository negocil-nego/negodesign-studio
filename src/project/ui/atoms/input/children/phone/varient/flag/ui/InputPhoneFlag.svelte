<script lang="ts">
  import { FlagNamed } from "../../../../../../../../../hooks/flag.svelte";
  import type { InputPhoneFlagProps } from "../data/types";
  import Icon from "../../../../../../icon/ui/Icon.svelte";
  import CountryPhonePopover from "../../../../../../../molecules/popover/ui/CountryPhonePopover.svelte";
  import type { CountryPhoneOption } from "../../../../../../../molecules/popover/data/country-phone";
  import Label from "../../../../../../label/ui/Label.svelte";
  import { StyleComponent } from "../../../../../../../../data/style.component";

  let { labelProps, inputProps, flag, locale, onCountrySelect }: InputPhoneFlagProps = $props();
  const style = StyleComponent.input.phone;
  let selectedFlag = $state<FlagNamed>(FlagNamed.Ao);

  $effect(() => {
    selectedFlag = flag[0] ?? FlagNamed.Ao;
  });

  function handleCountrySelect(country: CountryPhoneOption) {
    selectedFlag = country.flag;
    onCountrySelect?.(country);
  }
</script>

<div class={style.field}>
  {#if labelProps}
    <Label {...labelProps} />
  {/if}
  <div class={style.form}>
    <CountryPhonePopover flag={selectedFlag} {locale} onselect={handleCountrySelect} />
    <Icon name={inputProps.icon ?? "Phone"} className="shrink-0 text-zinc-400" />
    <input
      id={inputProps.id}
      type="tel"
      required={inputProps.required}
      disabled={inputProps.disabled}
      placeholder={inputProps.placeholder}
      value={inputProps.value}
      data-name="label"
    />
  </div>
</div>


