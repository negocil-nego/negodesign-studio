import 'package:flutter/material.dart';
import '../data/types.dart';

class Text extends StatelessWidget {
  final TextProps props;

  const Text(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Semantics(child: Text(props.Text));
  }
}
