import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../shared/base/ui/ButtonBase.dart';

/// Aproximação em Flutter do OutlineButton: reutiliza o ButtonBase.
/// A classe Tailwind `border border-primary/40` corresponde a um botão com
/// contorno; usamos o ButtonBase nativo.
class OutlineButton extends StatelessWidget {
  final OutlineButtonProps props;
  const OutlineButton(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return ButtonBase(props);
  }
}
