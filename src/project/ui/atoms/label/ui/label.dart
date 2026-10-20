import 'package:flutter/material.dart';
import '../data/types.dart';

class Label extends StatelessWidget {
  final LabelProps props;

  const Label(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Semantics(label: props.htmlFor, child: Text(props.label));
  }
}
