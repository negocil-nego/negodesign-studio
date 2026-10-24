import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../skeleton/ui/Skeleton.dart';

/// Aproximação em Flutter do átomo Avatar.
/// Quando `isLoading` é verdadeiro apresenta o Skeleton; caso contrário
/// apresenta a imagem a partir de `src` dentro de um botão clicável.
class Avatar extends StatelessWidget {
  final AvatarProps props;
  const Avatar(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    if (props.isLoading) {
      return Skeleton(SkeletonProps(className: props.className ?? 'w-12 h-12 rounded-lg'));
    }
    if (props.src != null) {
      return GestureDetector(
        onTap: props.onClick,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(8),
          child: Image.network(
            props.src!,
            width: 48,
            height: 48,
            fit: BoxFit.cover,
          ),
        ),
      );
    }
    return const SizedBox.shrink();
  }
}
