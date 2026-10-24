import 'package:flutter/material.dart';
import '../data/types.dart';

/// Aproximação em Flutter do elemento `<video>`: como o VideoPlayer não é
/// necessário aqui, mostramos o poster (ou um fundo neutro) com um ícone de play.
class CardVideo extends StatelessWidget {
  final CardVideoProps props;

  const CardVideo(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 200,
      width: double.infinity,
      clipBehavior: Clip.hardEdge,
      decoration: BoxDecoration(borderRadius: BorderRadius.circular(6)),
      child: Stack(
        fit: StackFit.expand,
        children: [
          if (props.poster != null)
            Image.network(
              props.poster!,
              fit: BoxFit.cover,
              errorBuilder: (context, error, stackTrace) =>
                  Container(color: const Color(0xFFE5E7EB)),
            )
          else
            Container(color: const Color(0xFFE5E7EB)),
          const Center(
            child: Icon(Icons.play_arrow, size: 40, color: Colors.white70),
          ),
        ],
      ),
    );
  }
}
