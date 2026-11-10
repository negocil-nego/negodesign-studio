import 'package:flutter/material.dart';
import '../data/country-phone.dart';
import '../data/viewmodel.dart';
import 'flutter/country_phone_dialog.dart';

class CountryPhonePopover extends StatelessWidget {
  final String flag;
  final ValueChanged<CountryPhoneOption>? onSelect;
  final bool embedded;

  const CountryPhonePopover({
    required this.flag,
    this.onSelect,
    this.embedded = false,
    super.key,
  });

  Future<void> _openPicker(BuildContext context) async {
    final result = await showDialog<CountryPhoneOption>(
      context: context,
      builder: (_) => CountryPhoneDialog(initialFlag: flag),
    );
    if (result != null) onSelect?.call(result);
  }

  @override
  Widget build(BuildContext context) {
    final vm = CountryPhonePopoverViewModel(flag: flag);
    final selected = vm.selected;
    return OutlinedButton(
      onPressed: () => _openPicker(context),
      style: embedded
          ? OutlinedButton.styleFrom(
              padding: const EdgeInsets.symmetric(horizontal: 8),
              side: BorderSide.none,
              shape: const RoundedRectangleBorder(borderRadius: BorderRadius.zero),
            )
          : null,
      child: Row(mainAxisSize: MainAxisSize.min, children: [
        Text(CountryPhonePopoverViewModel.emoji(selected.iso2), style: const TextStyle(fontSize: 18)),
        const SizedBox(width: 6),
        Text('+${selected.dialCode}'),
        const Icon(Icons.arrow_drop_down),
      ]),
    );
  }
}
