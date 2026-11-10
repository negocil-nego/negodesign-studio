import 'package:flutter/foundation.dart';

class InputRangeProps {
  final String? id;
  final String? label;
  final double value;
  final double min;
  final double max;
  final double step;
  final bool disabled;
  final bool showValue;
  final String? hint;
  final ValueChanged<double>? onValueChange;

  const InputRangeProps({
    this.id,
    this.label,
    this.value = 50,
    this.min = 0,
    this.max = 100,
    this.step = 1,
    this.disabled = false,
    this.showValue = true,
    this.hint,
    this.onValueChange,
  });
}
