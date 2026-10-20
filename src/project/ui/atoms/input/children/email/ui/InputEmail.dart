import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../core/ui/Input.dart';
import '../../label/ui/InputLabel.dart';

class InputEmail extends StatelessWidget {
  final InputEmailProps props;

  const InputEmail(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final emailProps = props.inputProps.copyWith(type: 'email');

    if (props.labelProps != null) {
      return InputLabel(
        InputLabelProps(labelProps: props.labelProps!, inputProps: emailProps),
      );
    } else {
      return Input(emailProps);
    }
  }
}
