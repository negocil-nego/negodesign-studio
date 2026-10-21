import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../../../molecules/popover/ui/CountryPhonePopover.dart';
import '../../../../../molecules/popover/data/country-phone.dart';
import '../../../../label/ui/Label.dart';

class InputPhoneFlag extends StatefulWidget {
  final InputPhoneFlagProps props;

  const InputPhoneFlag(this.props, {super.key});

  @override
  State<InputPhoneFlag> createState() => _InputPhoneFlagState();
}

class _InputPhoneFlagState extends State<InputPhoneFlag> {
  late final TextEditingController _controller;
  late String selectedFlag = widget.props.flag.isNotEmpty ? widget.props.flag.first : 'ao.svg';

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController(text: widget.props.inputProps.value ?? "");
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void selectCountry(CountryPhoneOption country) {
    setState(() {
      selectedFlag = country.flag;
    });
    widget.props.onCountrySelect?.call(country);
  }

  @override
  Widget build(BuildContext context) {
    final props = widget.props.inputProps;

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (widget.props.labelProps != null) ...[
          Label(widget.props.labelProps!),
          const SizedBox(height: 4),
        ],
        TextField(
          enabled: !props.disabled,
          keyboardType: TextInputType.phone,
          controller: _controller,
          decoration: InputDecoration(
            hintText: props.placeholder,
            prefixIcon: CountryPhonePopover(
              flag: selectedFlag,
              embedded: true,
              onSelect: selectCountry,
            ),
            prefixIconConstraints: const BoxConstraints(
              minWidth: 112,
              maxWidth: 148,
              minHeight: 48,
            ),
            contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
            border: const OutlineInputBorder(),
          ),
        ),
      ],
    );
  }
}




