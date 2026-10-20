import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../core/ui/Input.dart';
import '../../label/ui/InputLabel.dart';

class InputPhone extends StatelessWidget {
  final InputPhoneProps props;

  const InputPhone(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final phoneProps = props.inputProps.copyWith(type: 'tel');

    if (props.labelProps != null) {
      return InputLabel(
        InputLabelProps(labelProps: props.labelProps!, inputProps: phoneProps),
      );
    } else {
      return Input(phoneProps);
    }
  }
}
