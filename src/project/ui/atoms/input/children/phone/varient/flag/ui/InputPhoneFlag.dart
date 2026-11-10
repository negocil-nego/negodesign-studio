import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';
import '../../../../../../../molecules/popover/ui/CountryPhonePopover.dart';
import '../../../../../../label/ui/Label.dart';

class InputPhoneFlag extends StatefulWidget {
  final InputPhoneFlagProps props;
  const InputPhoneFlag(this.props, {super.key});

  @override
  State<InputPhoneFlag> createState() => _InputPhoneFlagState();
}

class _InputPhoneFlagState extends State<InputPhoneFlag> {
  late final InputPhoneFlagViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = InputPhoneFlagViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant InputPhoneFlag oldWidget) {
    super.didUpdateWidget(oldWidget);
    _vm.didUpdate(widget.props);
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final props = widget.props.inputProps;
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (widget.props.labelProps != null) ...[
            Label(widget.props.labelProps!),
            const SizedBox(height: 4),
          ],
          TextField(
            enabled: !props.disabled,
            keyboardType: TextInputType.phone,
            controller: _vm.controller,
            decoration: InputDecoration(
              hintText: props.placeholder,
              prefixIcon: CountryPhonePopover(
                flag: _vm.selectedFlag,
                embedded: true,
                onSelect: _vm.selectCountry,
              ),
              prefixIconConstraints: const BoxConstraints(minWidth: 112, maxWidth: 148, minHeight: 48),
              contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
              border: const OutlineInputBorder(),
            ),
          ),
        ],
      ),
    );
  }
}
