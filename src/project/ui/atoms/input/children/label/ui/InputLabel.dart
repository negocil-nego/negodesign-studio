import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../core/ui/input.dart';
import '../../../../label/ui/Label.dart';

class InputLabel extends StatelessWidget {
  final InputLabelProps props;

  const InputLabel(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Label(props.labelProps),
        const SizedBox(height: 4),
        Input(props.inputProps),
      ],
    );
  }
}
