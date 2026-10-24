import 'package:flutter/material.dart';
import '../data/types.dart';

class CardImage extends StatelessWidget {
  final CardImageProps props;

  const CardImage(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Image.network(
      props.src,
      width: double.infinity,
      fit: BoxFit.cover,
      semanticLabel: props.alt,
      errorBuilder: (context, error, stackTrace) => const SizedBox(height: 200),
      loadingBuilder: (context, child, progress) =>
          progress == null ? child : const SizedBox(height: 200),
    );
  }
}
