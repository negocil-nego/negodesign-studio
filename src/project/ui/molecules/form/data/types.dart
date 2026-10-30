import 'package:flutter/material.dart';

class FormLabelProps { final String label; final String forId; const FormLabelProps({required this.label, required this.forId}); }
class FormInputProps { final String? id, name, value, placeholder; final String type; final bool required, disabled; const FormInputProps({this.id, this.name, this.value, this.placeholder, this.type='text', this.required=false, this.disabled=false}); }
class FormOption { final String value, label; final bool disabled; const FormOption({required this.value, required this.label, this.disabled=false}); }
class FormIt {
  final String? component, type, id, name, value, placeholder, label;
  final bool required, disabled, checked;
  final int? rows;
  final FormLabelProps? labelProps;
  final FormInputProps? inputProps;
  final List<FormOption> options;
  const FormIt({this.component, this.type, this.id, this.name, this.value, this.placeholder, this.label, this.required=false, this.disabled=false, this.checked=false, this.rows, this.labelProps, this.inputProps, this.options=const []});
  String get effectiveLabel => labelProps?.label ?? label ?? '';
  String get effectiveType => inputProps?.type ?? type ?? component ?? 'text';
  String? get effectiveHint => inputProps?.placeholder ?? placeholder;
}
class FormProps {
  final String style;
  final String? className;
  final List<FormIt> fields;
  final VoidCallback? onSubmit;
  const FormProps({this.style='grid', this.className, this.fields=const [], this.onSubmit});
}
