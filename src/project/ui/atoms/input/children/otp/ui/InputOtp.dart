import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';

class InputOtp extends StatefulWidget {
  final InputOtpProps props;
  const InputOtp(this.props, {super.key});

  @override
  State<InputOtp> createState() => _InputOtpState();
}

class _InputOtpState extends State<InputOtp> {
  late final InputOtpViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = InputOtpViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant InputOtp oldWidget) {
    super.didUpdateWidget(oldWidget);
    _vm.didUpdate(oldWidget.props, widget.props);
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final border = OutlineInputBorder(borderRadius: BorderRadius.circular(8), borderSide: BorderSide(color: theme.colorScheme.outlineVariant));

    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) {
        final otp = Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          if (widget.props.labelProps != null) ...[
            SizedBox(
              width: _vm.isCentered ? _vm.groupWidth : null,
              child: Text(widget.props.labelProps!.label, textAlign: _vm.isCentered ? TextAlign.center : TextAlign.start, style: theme.textTheme.labelMedium),
            ),
            const SizedBox(height: 8),
          ],
          Semantics(
            label: widget.props.labelProps?.label ?? 'Verification code',
            child: Row(mainAxisSize: MainAxisSize.min, children: [
              for (var i = 0; i < _vm.length; i++) ...[
                if (_vm.shouldShowSeparator(i)) Padding(padding: const EdgeInsets.symmetric(horizontal: 4), child: Text('•', style: TextStyle(color: theme.hintColor))),
                SizedBox(
                  width: 44,
                  height: 44,
                  child: TextField(
                    controller: _vm.controllers[i],
                    focusNode: _vm.focusNodes[i],
                    enabled: !widget.props.disabled,
                    keyboardType: TextInputType.number,
                    textAlign: TextAlign.center,
                    maxLength: 1,
                    inputFormatters: [FilteringTextInputFormatter.digitsOnly, LengthLimitingTextInputFormatter(1)],
                    decoration: InputDecoration(
                      counterText: '',
                      filled: true,
                      fillColor: theme.colorScheme.surfaceContainerHighest,
                      contentPadding: EdgeInsets.zero,
                      enabledBorder: border,
                      focusedBorder: border.copyWith(borderSide: BorderSide(color: theme.colorScheme.primary, width: 1.5)),
                      disabledBorder: border,
                    ),
                    onChanged: (d) => _vm.updateDigit(i, d),
                  ),
                ),
              ],
            ]),
          ),
        ]);
        return _vm.isCentered ? Center(child: otp) : otp;
      },
    );
  }
}
