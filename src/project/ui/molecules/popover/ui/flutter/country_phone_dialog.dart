import 'package:flutter/material.dart';
import '../../data/country-phone.dart';
import '../../data/viewmodel.dart';

class CountryPhoneDialog extends StatefulWidget {
  final String initialFlag;
  const CountryPhoneDialog({super.key, required this.initialFlag});

  @override
  State<CountryPhoneDialog> createState() => _CountryPhoneDialogState();
}

class _CountryPhoneDialogState extends State<CountryPhoneDialog> {
  late final CountryPhonePopoverViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = CountryPhonePopoverViewModel(flag: widget.initialFlag);
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) {
        final matches = _vm.filteredCountries;
        return AlertDialog(
          content: SizedBox(
            width: 360,
            height: 420,
            child: Column(children: [
              TextField(
                decoration: const InputDecoration(hintText: 'Search country or code'),
                onChanged: _vm.updateQuery,
              ),
              const SizedBox(height: 8),
              Expanded(
                child: ListView(
                  children: matches
                      .map(
                        (country) => ListTile(
                          leading: Text(CountryPhonePopoverViewModel.emoji(country.iso2), style: const TextStyle(fontSize: 18)),
                          title: Text(country.name),
                          trailing: Text('+${country.dialCode}'),
                          onTap: () => Navigator.of(context).pop(country),
                        ),
                      )
                      .toList(),
                ),
              ),
            ]),
          ),
        );
      },
    );
  }
}

