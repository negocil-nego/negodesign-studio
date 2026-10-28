import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../data/types.dart';

class InputOtp extends StatefulWidget {
  final InputOtpProps props;

  const InputOtp(this.props, {super.key});

  @override
  State<InputOtp> createState() => _InputOtpState();
}

class _InputOtpState extends State<InputOtp> {
  late List<TextEditingController> controllers;
  late List<FocusNode> focusNodes;

  int get length => widget.props.length.clamp(1, 12).toInt();

  @override
  void initState() {
    super.initState();
    _createFields();
  }

  void _createFields() {
    final digits = widget.props.value.replaceAll(RegExp(r'\D'), '').split('');
    controllers = List.generate(length, (index) => TextEditingController(text: index < digits.length ? digits[index] : ''));
    focusNodes = List.generate(length, (_) => FocusNode());
  }

  @override
  void didUpdateWidget(covariant InputOtp oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.props.length != widget.props.length) {
      _disposeFields();
      _createFields();
    } else if (oldWidget.props.value != widget.props.value) {
      final digits = widget.props.value.replaceAll(RegExp(r'\D'), '').split('');
      for (var index = 0; index < length; index++) {
        controllers[index].text = index < digits.length ? digits[index] : '';
      }
    }
  }

  void _disposeFields() {
    for (final controller in controllers) { controller.dispose(); }
    for (final node in focusNodes) { node.dispose(); }
  }

  @override
  void dispose() {
    _disposeFields();
    super.dispose();
  }

  void _update(int index, String digit) {
    controllers[index].text = digit;
    widget.props.onValueChange?.call(controllers.map((controller) => controller.text).join());
    if (digit.isNotEmpty && index < length - 1) focusNodes[index + 1].requestFocus();
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final border = OutlineInputBorder(
      borderRadius: BorderRadius.circular(8),
      borderSide: BorderSide(color: theme.colorScheme.outlineVariant),
    );

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (widget.props.labelProps != null) ...[
          Text(widget.props.labelProps!.label, style: theme.textTheme.labelMedium),
          const SizedBox(height: 8),
        ],
        Semantics(
          label: widget.props.labelProps?.label ?? 'Código de verificação',
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              for (var index = 0; index < length; index++) ...[
                if (widget.props.separatorEvery > 0 && index > 0 && index % widget.props.separatorEvery == 0)
                  Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 4),
                    child: Text('•', style: TextStyle(color: theme.hintColor)),
                  ),
                SizedBox(
                  width: 44,
                  height: 44,
                  child: TextField(
                    controller: controllers[index],
                    focusNode: focusNodes[index],
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
                    onChanged: (digit) => _update(index, digit),
                  ),
                ),
              ],
            ],
          ),
        ),
      ],
    );
  }
}
