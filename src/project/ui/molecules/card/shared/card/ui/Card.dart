import 'package:flutter/material.dart';
import '../data/types.dart';

/// Aproximação em Flutter do componente Card.
/// Container raiz identificado por `id` "card".
class Card extends StatelessWidget {
  final CardProps props;

  const Card(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      key: const Key('card'),
      decoration: BoxDecoration(borderRadius: BorderRadius.circular(12)),
      child: const SizedBox.shrink(),
    );
  }
}
