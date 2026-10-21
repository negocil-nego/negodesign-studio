import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../core/data/types.dart';
import '../../../core/ui/Input.dart';
import '../../label/ui/InputLabel.dart';

class InputDate extends StatelessWidget {
  final InputDateProps props;

  const InputDate(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final componentProps = props.inputProps.copyWith(type: InputType.date);

    if (props.labelProps != null) {
      return InputLabel(
        InputLabelProps(labelProps: props.labelProps!, inputProps: componentProps),
      );
    }

    return Input(componentProps);
  }
}
