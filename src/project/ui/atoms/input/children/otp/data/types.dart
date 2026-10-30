import 'package:flutter/material.dart';

class InputOtpLabelProps {
  final String label;
  final String htmlFor;

  const InputOtpLabelProps({required this.label, required this.htmlFor});
}

class InputOtpProps {
  final String? id;
  final String? name;
  final String value;
  final int length;
  final int separatorEvery;
  final InputOtpLabelProps? labelProps;
  final bool disabled;
  final bool required;
  final String position;
  final ValueChanged<String>? onValueChange;

  const InputOtpProps({
    this.id,
    this.name,
    this.value = '',
    this.length = 6,
    this.separatorEvery = 3,
    this.labelProps,
    this.disabled = false,
    this.required = false,
    this.position = 'start',
    this.onValueChange,
  });
}
