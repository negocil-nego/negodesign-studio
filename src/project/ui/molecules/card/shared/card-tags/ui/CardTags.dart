import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../../../atoms/icon/data/types.dart';
import '../../../../../atoms/icon/ui/Icon.dart' as negodesign;

/// Aproximação em Flutter do componente CardTags: o ícone (hugeicons) não tem
/// equivalente directo em Material, pelo que é representado por um ícone genérico.
class CardTags extends StatelessWidget {
  final CardTagsProps props;

  const CardTags(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    if (props.variant == 'buttons') {
      return Wrap(
        spacing: 10,
        runSpacing: 8,
        children: props.tags.map((tag) => Container(
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
          decoration: BoxDecoration(
            border: Border.all(color: Colors.grey.shade400),
            borderRadius: BorderRadius.circular(24),
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              if (tag.icon != null) ...[
                negodesign.Icon(IconProps(name: tag.icon!)),
                const SizedBox(width: 8),
              ],
              Text(tag.title, style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w500)),
            ],
          ),
        )).toList(),
      );
    }

    return Row(
      mainAxisSize: MainAxisSize.min,
      children: props.tags.map((tag) {
        return Padding(
          padding: const EdgeInsets.only(right: 24),
          child: Row(
            children: [
              if (tag.icon != null) ...[negodesign.Icon(IconProps(name: tag.icon!)), const SizedBox(width: 8)],
              Text(tag.title, style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w600)),
              if (tag.description != null)
                Text(
                  ' ${tag.description!}',
                  style: const TextStyle(fontSize: 12, color: Color(0xFF9CA3AF)),
                ),
            ],
          ),
        );
      }).toList(),
    );
  }
}
