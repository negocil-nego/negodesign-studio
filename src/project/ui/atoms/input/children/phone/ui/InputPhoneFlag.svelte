<script lang="ts">
  import { getFlagUrl } from "../../../../../../../hooks/flag.svelte";
  import Input from "../../../core/ui/Input.svelte";
  import InputLabel from "../../label/ui/InputLabel.svelte";
  import type { InputPhoneFlagProps } from "../data/types";
  import CountryPhonePopover from "../../../../../molecules/popover/ui/CountryPhonePopover.svelte";
  import type { CountryPhoneOption } from "../../../../../molecules/popover/data/country-phone";
  import { FlagNamed } from "../../../../../../../hooks/flag.svelte";

  let { labelProps, inputProps, flag, locale, onCountrySelect }: InputPhoneFlagProps = $props();
  let selectedFlag = $state(flag[0] ?? FlagNamed.Ao);
  let selectedDialCode = $state("");
  const phoneProps = $derived({
    ...inputProps,
    type: "tel" as const,
    value: selectedDialCode ? `+${selectedDialCode} ${inputProps.value ?? ""}` : inputProps.value,
  });

  function handleCountrySelect(country: CountryPhoneOption) {
    selectedFlag = country.flag;
    selectedDialCode = country.dialCode;
    onCountrySelect?.(country);
  }
</script>

<div class="input-phone-flag">
  <CountryPhonePopover flag={selectedFlag} {locale} onselect={handleCountrySelect} />
  {#each flag.slice(1) as selectedFlagItem (selectedFlagItem)}
    <img src={getFlagUrl(selectedFlagItem)} alt={selectedFlagItem} />
  {/each}
  {#if labelProps}
    <InputLabel {labelProps} inputProps={phoneProps} />
  {:else}
    <Input {...phoneProps} />
  {/if}
</div>

