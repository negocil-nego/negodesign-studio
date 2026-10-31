import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../core/data/types.dart';
import '../../../core/ui/Input.dart';
import '../../label/ui/InputLabel.dart';

class InputNumber extends StatelessWidget {
  final InputNumberProps props;

  const InputNumber(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final componentProps = props.inputProps.copyWith(type: InputType.number, icon: props.inputProps.icon ?? 'Number');

    if (props.labelProps != null) {
      return InputLabel(
        InputLabelProps(labelProps: props.labelProps!, inputProps: componentProps),
      );
    }

    return Input(componentProps);
  }
}
