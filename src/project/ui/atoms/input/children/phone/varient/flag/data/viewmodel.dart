import 'package:flutter/material.dart';
import '../../../../../../../molecules/popover/data/country-phone.dart';
import 'types.dart';

class InputPhoneFlagViewModel extends ChangeNotifier {
  InputPhoneFlagProps props;
  late final TextEditingController controller;
  late String selectedFlag;

  InputPhoneFlagViewModel(this.props) {
    controller = TextEditingController(text: props.inputProps.value ?? '');
    selectedFlag = props.flag.isNotEmpty ? props.flag.first : 'ao.svg';
  }

  void didUpdate(InputPhoneFlagProps newProps) {
    props = newProps;
    if ((newProps.inputProps.value ?? '') != controller.text) {
      controller.text = newProps.inputProps.value ?? '';
    }
  }

  void selectCountry(CountryPhoneOption country) {
    selectedFlag = country.flag;
    props.onCountrySelect?.call(country);
    notifyListeners();
  }

  @override
  void dispose() {
    controller.dispose();
    super.dispose();
  }
}

