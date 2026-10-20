import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../core/ui/input.dart';
import '../../label/ui/input_label.dart';

class InputText extends StatelessWidget {
  final InputTextProps props;

  const InputText(this.props, {super.key});

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
