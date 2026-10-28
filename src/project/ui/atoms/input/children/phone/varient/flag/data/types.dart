import "../../../../../../input/core/data/types.dart" show InputProps;
import "../../../../../../label/data/types.dart" show LabelProps;
import "../../../../../../../molecules/popover/data/country-phone.dart" show CountryPhoneOption;

class InputPhoneFlagProps {
  final LabelProps? labelProps;
  final InputProps inputProps;
  final List<String> flag;
  final void Function(CountryPhoneOption country)? onCountrySelect;

  const InputPhoneFlagProps({
    this.labelProps,
    required this.inputProps,
    required this.flag,
    this.onCountrySelect,
  });
}
