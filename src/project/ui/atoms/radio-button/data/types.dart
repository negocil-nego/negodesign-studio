import 'package:flutter/foundation.dart';

class RadioButtonOption {
  final String value;
  final String label;
  final bool disabled;

  const RadioButtonOption({
    required this.value,
    required this.label,
    this.disabled = false,
  });
}

class RadioButtonProps {
  final List<RadioButtonOption> options;
  final String value;
  final bool multiple;
  final List<String> values;
  final bool disabled;
  final ValueChanged<String>? onValueChange;

  const RadioButtonProps({
    required this.options,
    this.value = '',
    this.multiple = false,
    this.values = const [],
    this.disabled = false,
    this.onValueChange,
  });
}
