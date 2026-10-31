import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../core/data/types.dart';
import '../../../core/ui/Input.dart';
import '../../label/ui/InputLabel.dart';

class InputAge extends StatelessWidget {
  final InputAgeProps props;

  const InputAge(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final componentProps = props.inputProps.copyWith(type: InputType.number, icon: props.inputProps.icon ?? 'Age');

    if (props.labelProps != null) {
      return InputLabel(
        InputLabelProps(labelProps: props.labelProps!, inputProps: componentProps),
      );
    }

    return Input(componentProps);
  }
}
