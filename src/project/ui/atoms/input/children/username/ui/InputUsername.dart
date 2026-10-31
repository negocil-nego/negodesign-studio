import 'package:flutter/material.dart';
import '../../../icon/ui/Icon.dart' as project_icons;
import '../data/types.dart';

class InputUsername extends StatelessWidget {
  final InputUsernameProps props;

  const InputUsername(this.props, {super.key});

  static final _username = RegExp(r'^[\p{L}\p{N}][\p{L}\p{N}._-]{2,29}$', unicode: true);
  static final _email = RegExp(r'^[^\s@]+@[^\s@]+\.[^\s@]{2,}$');
  static final _phone = RegExp(r'^\+?[1-9]\d{6,14}$');

  bool _isValid(String? value) {
    final candidate = (value ?? '').trim();
    if (candidate.isEmpty) return !props.inputProps.required;
    final normalizedPhone = candidate.replaceAll(RegExp(r'[\s().-]'), '');
    final matchesUsername = _username.hasMatch(candidate);
    final matchesEmail = _email.hasMatch(candidate);
    final matchesPhone = _phone.hasMatch(normalizedPhone);

    return switch (props.validationMode) {
      UsernameValidationMode.username => matchesUsername,
      UsernameValidationMode.email => matchesEmail,
      UsernameValidationMode.phone => matchesPhone,
      UsernameValidationMode.usernameOrEmail => matchesUsername || matchesEmail,
      UsernameValidationMode.usernameOrPhone => matchesUsername || matchesPhone,
      UsernameValidationMode.any => matchesUsername || matchesEmail || matchesPhone,
    };
  }

  String get _errorMessage => props.errorMessage ?? switch (props.validationMode) {
    UsernameValidationMode.username => 'Introduza um nome de utilizador válido.',
    UsernameValidationMode.email => 'Introduza um email válido.',
    UsernameValidationMode.phone => 'Introduza um telefone válido com o código do país.',
    UsernameValidationMode.usernameOrEmail => 'Introduza um nome de utilizador ou email válido.',
    UsernameValidationMode.usernameOrPhone => 'Introduza um nome de utilizador ou telefone válido.',
    UsernameValidationMode.any => 'Introduza um nome de utilizador, email ou telefone válido.',
  };

  @override
  Widget build(BuildContext context) {
    return TextFormField(
      initialValue: props.inputProps.value,
      enabled: !props.inputProps.disabled,
      keyboardType: TextInputType.text,
      onChanged: props.onValueChange,
      validator: (value) {
        final valid = _isValid(value);
        props.onValidityChange?.call(valid);
        return valid ? null : _errorMessage;
      },
      decoration: InputDecoration(
        labelText: props.labelProps?.label,
        hintText: props.inputProps.placeholder ?? 'Nome de utilizador, email ou telefone',
        prefixIcon: project_icons.Icon(project_icons.IconProps(name: props.inputProps.icon ?? 'User')),
        errorBorder: const OutlineInputBorder(borderSide: BorderSide(color: Colors.red)),
        focusedErrorBorder: const OutlineInputBorder(borderSide: BorderSide(color: Colors.red, width: 2)),
      ),
    );
  }
}
