import type { FlagNamed } from "../../../../../../../../../hooks/flag.svelte";
import type { CountryPhoneOption } from "../../../../../../../molecules/popover/data/country-phone";
import type { InputPhoneProps } from "../../../data/types";

export type InputPhoneFlagProps = InputPhoneProps & {
  flag: FlagNamed[];
  locale?: string;
  onCountrySelect?: (country: CountryPhoneOption) => void;
};
