import 'package:flutter/material.dart';
import 'types.dart';

class FormViewModel {
  const FormViewModel();

  bool isDisabled(FormIt field) {
    return field.disabled || (field.inputProps?.disabled ?? false);
  }

  bool isRequired(FormIt field) {
    return field.required || (field.inputProps?.required ?? false);
  }

  TextInputType? keyboardTypeFor(String type) {
    if (type.contains('email')) return TextInputType.emailAddress;
    if (type.contains('phone') || type == 'tel') return TextInputType.phone;
    return null;
  }

  FormFieldValidator<String>? validatorFor(FormIt field) {
    if (!isRequired(field)) return null;
    return (value) => value == null || value.isEmpty ? 'Campo obrigatório' : null;
  }
}

