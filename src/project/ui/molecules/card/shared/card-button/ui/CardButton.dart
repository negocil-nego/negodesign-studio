import 'package:flutter/material.dart';
import '../../../../../atoms/button/variant/default/ui/Button.dart';
import '../data/types.dart';

/// Aproximação em Flutter do componente CardButton.
/// Envolve o botão por omissão (`atoms/button/variant/default`), herdando o
/// comportamento das variantes `solid` (por omissão) e `outline`.
class CardButton extends StatelessWidget {
  final CardButtonProps props;

  const CardButton(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Button(props);
  }
}
