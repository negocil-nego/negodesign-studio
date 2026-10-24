import 'package:flutter/material.dart';
import '../data/types.dart';

/// Aproximação em Flutter do componente CardActionsButton.
/// Renderiza uma linha de botões (outline) com o ícone e o rótulo de cada acção.
class CardActionsButton extends StatelessWidget {
  final CardActionsButtonProps props;

  const CardActionsButton(this.props, {super.key});

  /// Mapeia o nome semântico do ícone para o ícone Material equivalente.
  IconData _iconFor(String? name) {
    switch (name) {
      case 'Cart':
        return Icons.shopping_cart_outlined;
      case 'Favorite':
        return Icons.favorite_border;
      case 'Share':
        return Icons.share_outlined;
      case 'Delete':
        return Icons.delete_outline;
      case 'Edit':
        return Icons.edit_outlined;
      case 'Add':
        return Icons.add;
      case 'Close':
        return Icons.close;
      default:
        return Icons.circle_outlined;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        for (var i = 0; i < props.actions.length; i++) ...[
          if (i > 0) const SizedBox(width: 12),
          Expanded(
            child: OutlinedButton(
              onPressed: props.actions[i].onClick,
              style: OutlinedButton.styleFrom(
                padding: const EdgeInsets.symmetric(vertical: 10),
                foregroundColor: const Color(0xFF111827),
                side: const BorderSide(color: Color(0xFFE5E7EB)),
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
              ),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  if (props.actions[i].icon != null) ...[
                    Icon(_iconFor(props.actions[i].icon), size: 18),
                    const SizedBox(width: 8),
                  ],
                  Text(
                    props.actions[i].label,
                    style: const TextStyle(fontWeight: FontWeight.w600),
                  ),
                ],
              ),
            ),
          ),
        ],
      ],
    );
  }
}
