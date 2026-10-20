import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../core/data/types.dart' show InputType;
import '../../../core/ui/Input.dart';
import '../../label/ui/InputLabel.dart';
import '../../../../../molecules/popover/ui/CountryPhonePopover.dart';
import '../../../../../molecules/popover/data/country-phone.dart';

class InputPhoneFlag extends StatefulWidget {
  final InputPhoneFlagProps props;

  const InputPhoneFlag(this.props, {super.key});

  @override
  State<InputPhoneFlag> createState() => _InputPhoneFlagState();
}

class _InputPhoneFlagState extends State<InputPhoneFlag> {
  late String selectedFlag = widget.props.flag.isNotEmpty ? widget.props.flag.first : 'ao.svg';
  String dialCode = '';

  void selectCountry(CountryPhoneOption country) {
    setState(() {
      selectedFlag = country.flag;
      dialCode = country.dialCode;
    });
    widget.props.onCountrySelect?.call(country);
  }

  String _emoji(String iso2) => String.fromCharCodes(
    iso2.codeUnits.map((unit) => 0x1F1E6 + unit - 65),
  );

  @override
  Widget build(BuildContext context) {
    final originalValue = widget.props.inputProps.value ?? '';
    final phoneProps = widget.props.inputProps.copyWith(
      type: InputType.tel,
      value: dialCode.isEmpty ? originalValue : '+$dialCode $originalValue',
    );
    final input = widget.props.labelProps != null
        ? InputLabel(InputLabelProps(labelProps: widget.props.labelProps!, inputProps: phoneProps))
        : Input(phoneProps);

    return Row(children: [
      CountryPhonePopover(flag: selectedFlag, onSelect: selectCountry),
      const SizedBox(width: 8),
      if (widget.props.flag.length > 1)
        ...widget.props.flag.skip(1).map((flag) => Padding(
          padding: const EdgeInsets.only(right: 4),
          child: Text(_emoji(flag.substring(0, 2).toUpperCase()), style: const TextStyle(fontSize: 18)),
        )),
      Expanded(child: input),
    ]);
  }
}
