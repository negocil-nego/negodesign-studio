import "../../label/data/types.dart" show LabelProps;
import "../../../core/data/types.dart" show InputProps;
import "../../../../../molecules/popover/data/country-phone.dart" show CountryPhoneOption;

typedef InputPhoneProps = InputWithoutLabelProps;

class InputWithoutLabelProps {
  final LabelProps? labelProps;
  final InputProps inputProps;

  const InputWithoutLabelProps({this.labelProps, required this.inputProps});
}

class InputPhoneFlagProps {
  final LabelProps? labelProps;
  final InputProps inputProps;
  final List<String> flag;
  final void Function(CountryPhoneOption country)? onCountrySelect;

  const InputPhoneFlagProps({this.labelProps, required this.inputProps, required this.flag, this.onCountrySelect});
}
