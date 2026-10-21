import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../core/data/types.dart';
import '../../../core/ui/Input.dart';
import '../../label/ui/InputLabel.dart';

class InputTime extends StatelessWidget {
  final InputTimeProps props;

  const InputTime(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final componentProps = props.inputProps.copyWith(type: InputType.time);

    if (props.labelProps != null) {
      return InputLabel(
        InputLabelProps(labelProps: props.labelProps!, inputProps: componentProps),
      );
    }

    return Input(componentProps);
  }
}
