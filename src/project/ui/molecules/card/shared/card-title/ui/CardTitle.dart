import 'package:flutter/material.dart';
import '../data/types.dart';

class CardTitle extends StatelessWidget {
  final CardTitleProps props;

  const CardTitle(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Text(
      props.title,
      style: const TextStyle(fontSize: 18, fontWeight: FontWeight.w600),
    );
  }
}
