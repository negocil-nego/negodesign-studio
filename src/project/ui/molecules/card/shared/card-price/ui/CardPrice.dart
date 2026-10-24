import 'package:flutter/material.dart';
import '../data/types.dart';

class CardPrice extends StatelessWidget {
  final CardPriceProps props;

  const CardPrice(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Text(
      '${props.number}',
      style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
    );
  }
}
