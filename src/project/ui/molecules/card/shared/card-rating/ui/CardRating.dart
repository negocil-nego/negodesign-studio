import 'package:flutter/material.dart';
import '../data/types.dart';

class CardRating extends StatelessWidget {
  final CardRatingProps props;

  const CardRating(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    // Equivalente a clamp(round(rating), 0, 5)
    final filled = props.rating.round().clamp(0, 5).toInt();
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: List.generate(5, (index) {
        final isFilled = index < filled;
        return Padding(
          padding: const EdgeInsets.only(right: 8),
          child: Icon(
            isFilled ? Icons.star : Icons.star_border,
            color: isFilled ? const Color(0xFFCA8A04) : null,
            size: 20,
          ),
        );
      }),
    );
  }
}
