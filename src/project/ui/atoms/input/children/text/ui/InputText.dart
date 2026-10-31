import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../core/ui/input.dart';
import '../../label/ui/InputLabel.dart';

class InputText extends StatelessWidget {
  final InputTextProps props;

  const InputText(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final textProps = props.inputProps.copyWith(type: 'text', icon: props.inputProps.icon ?? 'InputCursorText');

    if (props.labelProps != null) {
      return InputLabel(
        InputLabelProps(labelProps: props.labelProps!, inputProps: textProps),
      );
    } else {
      return Input(textProps);
    }
  }
}
