import 'package:flutter/material.dart';
import '../data/country-phone.dart';

class CountryPhonePopover extends StatefulWidget {
  final String flag;
  final ValueChanged<CountryPhoneOption>? onSelect;
  final bool embedded;

  const CountryPhonePopover({required this.flag, this.onSelect, this.embedded = false, super.key});

  @override
  State<CountryPhonePopover> createState() => _CountryPhonePopoverState();
}

class _CountryPhonePopoverState extends State<CountryPhonePopover> {
  CountryPhoneOption get selected => countryPhoneOptions.firstWhere(
    (country) => country.flag == widget.flag,
    orElse: () => countryPhoneOptions.first,
  );

  String _emoji(String iso2) => String.fromCharCodes(
    iso2.codeUnits.map((unit) => 0x1F1E6 + unit - 65),
  );

  Future<void> _openPicker() async {
    final result = await showDialog<CountryPhoneOption>(
      context: context,
      builder: (context) => _CountryPhoneDialog(),
    );
    if (result != null) widget.onSelect?.call(result);
  }

  @override
  Widget build(BuildContext context) => OutlinedButton(
    onPressed: _openPicker,
    style: widget.embedded
        ? OutlinedButton.styleFrom(
            padding: const EdgeInsets.symmetric(horizontal: 8),
            side: BorderSide.none,
            shape: const RoundedRectangleBorder(borderRadius: BorderRadius.zero),
          )
        : null,
    child: Row(mainAxisSize: MainAxisSize.min, children: [
      Text(_emoji(selected.iso2), style: const TextStyle(fontSize: 18)),
      const SizedBox(width: 6),
      Text('+${selected.dialCode}'),
      const Icon(Icons.arrow_drop_down),
    ]),
  );
}

class _CountryPhoneDialog extends StatefulWidget {
  @override
  State<_CountryPhoneDialog> createState() => _CountryPhoneDialogState();
}

class _CountryPhoneDialogState extends State<_CountryPhoneDialog> {
  String query = '';

  String _emoji(String iso2) => String.fromCharCodes(
    iso2.codeUnits.map((unit) => 0x1F1E6 + unit - 65),
  );

  @override
  Widget build(BuildContext context) {
    final term = query.trim().toLowerCase().replaceFirst(RegExp(r'^\+'), '');
    final matches = countryPhoneOptions.where((country) =>
      country.name.toLowerCase().contains(term) ||
      country.iso2.toLowerCase().contains(term) ||
      country.dialCode.startsWith(term),
    ).toList();
    return AlertDialog(
      content: SizedBox(width: 360, height: 420, child: Column(children: [
        TextField(decoration: const InputDecoration(hintText: 'Search country or code'), onChanged: (value) => setState(() => query = value)),
        const SizedBox(height: 8),
        Expanded(child: ListView(children: matches.map((country) => ListTile(
          leading: Text(_emoji(country.iso2), style: const TextStyle(fontSize: 18)),
          title: Text(country.name),
          trailing: Text('+${country.dialCode}'),
          onTap: () => Navigator.of(context).pop(country),
        )).toList())),
      ])),
    );
  }
}


