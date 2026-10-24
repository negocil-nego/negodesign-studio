import 'package:flutter/material.dart';
import '../data/types.dart';

/// Aproximação em Flutter do componente CardFavoriteButton.
/// Botão circular com o ícone fixo "Favorite".
class CardFavoriteButton extends StatelessWidget {
  final CardFavoriteButtonProps props;

  const CardFavoriteButton(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: props.onClick,
      child: Container(
        width: 40,
        height: 40,
        decoration: BoxDecoration(
          color: const Color(0xFFF3F4F6),
          shape: BoxShape.circle,
          border: Border.all(color: const Color(0xFFF3F4F6)),
        ),
        child: const Center(child: Icon(Icons.favorite_border)),
      ),
    );
  }
}
