import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../card-image/ui/CardImage.dart';

class CardMinFile extends StatelessWidget {
  final CardMinFileProps props;

  const CardMinFile(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final active = props.active;
    return GestureDetector(
      onTap: props.onClick,
      child: Container(
        width: 56,
        height: 56,
        clipBehavior: Clip.hardEdge,
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(6),
          border: Border.all(
            color: active ? const Color(0xFF3B82F6) : const Color(0xFFE5E7EB),
            width: active ? 1.5 : 1,
          ),
        ),
        child: props.imageProps != null ? CardImage(props.imageProps!) : null,
      ),
    );
  }
}
