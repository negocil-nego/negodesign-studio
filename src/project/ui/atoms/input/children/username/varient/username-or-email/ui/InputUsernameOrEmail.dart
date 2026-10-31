import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../ui/InputUsername.dart';
import '../../../data/types.dart';

class InputUsernameOrEmail extends StatelessWidget {
  final InputUsernameOrEmailProps props;

  const InputUsernameOrEmail(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return InputUsername(
      InputUsernameProps(
        labelProps: props.labelProps,
        inputProps: props.inputProps.copyWith(icon: props.inputProps.icon ?? 'Email'),
        errorMessage: props.errorMessage,
        onValueChange: props.onValueChange,
        onValidityChange: props.onValidityChange,
        validationMode: UsernameValidationMode.usernameOrEmail,
      ),
    );
  }
}
