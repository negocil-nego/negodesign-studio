import 'package:flutter/material.dart';
import '../data/types.dart';

/// Aproximação em Flutter do componente CardDescription.
class CardDescription extends StatelessWidget {
  final CardDescriptionProps props;

  const CardDescription(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      key: const Key('card-description'),
      child: Text(
        props.description,
        textAlign: TextAlign.justify,
        maxLines: 2,
        overflow: TextOverflow.ellipsis,
      ),
    );
  }
}
