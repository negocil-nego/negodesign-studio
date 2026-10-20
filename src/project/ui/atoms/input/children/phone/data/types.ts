import type { InputWithoutLabelProps } from "../../label/data/types";
import type { FlagNamed } from "../../../../../../../hooks/flag.svelte";
import type { CountryPhoneOption } from "../../../../../molecules/popover/data/country-phone";

export type InputPhoneProps = InputWithoutLabelProps;

export type InputPhoneFlagProps = InputWithoutLabelProps & {
  flag: FlagNamed[];
  locale?: string;
  onCountrySelect?: (country: CountryPhoneOption) => void;
};
