import 'package:flutter/material.dart';
import '../data/types.dart';

/// Aproximação em Flutter do componente CardLogo.
/// Envolve a imagem do logótipo num `ClipRRect` clicável.
class CardLogo extends StatelessWidget {
  final CardLogoProps props;

  const CardLogo(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    if (props.isLoading) {
      return Container(
        width: 48,
        height: 48,
        decoration: BoxDecoration(
          color: const Color(0xFFE5E7EB),
          borderRadius: BorderRadius.circular(8),
        ),
      );
    }
    if (props.src == null) {
      return const SizedBox.shrink();
    }
    return GestureDetector(
      onTap: props.onClick,
      child: ClipRRect(
        borderRadius: BorderRadius.circular(8),
        child: Image.network(
          props.src!,
          width: 48,
          height: 48,
          fit: BoxFit.cover,
          semanticLabel: props.alt,
        ),
      ),
    );
  }
}
