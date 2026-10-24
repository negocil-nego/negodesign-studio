import 'package:flutter/material.dart';
import '../data/types.dart';

/// Aproximação em Flutter do átomo Skeleton.
/// Não existe equivalente directo das classes Tailwind (incluindo o efeito
/// de shimmer), pelo que usamos um Container com a mesma cor de fundo
/// (bg-gray-200) e o mesmo raio (rounded-xl) para representar o bloco.
class Skeleton extends StatelessWidget {
  final SkeletonProps props;
  const Skeleton(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final radius = props.className?.contains('rounded-full') == true ? 999.0 : 12.0;
    return Container(
      decoration: BoxDecoration(
        color: const Color(0xFFE5E7EB),
        borderRadius: BorderRadius.circular(radius),
      ),
    );
  }
}
