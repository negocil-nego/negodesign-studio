import 'package:flutter/material.dart';
import 'country-phone.dart';

class CountryPhonePopoverViewModel extends ChangeNotifier {
  String flag;
  String query = '';

  CountryPhonePopoverViewModel({required this.flag});

  CountryPhoneOption get selected => countryPhoneOptions.firstWhere(
        (country) => country.flag == flag,
        orElse: () => countryPhoneOptions.first,
      );

  static String emoji(String iso2) => String.fromCharCodes(
        iso2.codeUnits.map((unit) => 0x1F1E6 + unit - 65),
      );

  List<CountryPhoneOption> get filteredCountries {
    final term = query.trim().toLowerCase().replaceFirst(RegExp(r'^\+'), '');
    if (term.isEmpty) return countryPhoneOptions;
    return countryPhoneOptions
        .where(
          (country) =>
              country.name.toLowerCase().contains(term) ||
              country.iso2.toLowerCase().contains(term) ||
              country.dialCode.startsWith(term),
        )
        .toList();
  }

  void updateQuery(String value) {
    query = value;
    notifyListeners();
  }
}

