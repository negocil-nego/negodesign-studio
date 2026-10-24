import 'package:flutter/material.dart';
import '../data/types.dart';

/// Aproximação em Flutter do componente CardTags: o ícone (hugeicons) não tem
/// equivalente directo em Material, pelo que é representado por um ícone genérico.
class CardTags extends StatelessWidget {
  final CardTagsProps props;

  const CardTags(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Row(
      children: props.tags.map((tag) {
        return Expanded(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              if (tag.icon != null) ...[
                const Icon(Icons.circle_outlined, size: 20),
                const SizedBox(height: 4),
              ],
              Text(
                tag.title,
                style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold, color: Color(0xFF111827)),
              ),
              if (tag.description != null)
                Text(
                  tag.description!,
                  style: const TextStyle(fontSize: 14, color: Color(0xFF6B7280)),
                ),
            ],
          ),
        );
      }).toList(),
    );
  }
}
