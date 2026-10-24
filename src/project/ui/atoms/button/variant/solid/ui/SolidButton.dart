import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../shared/base/ui/ButtonBase.dart';

/// Aproximação em Flutter do SolidButton: reutiliza o ButtonBase.
/// A classe Tailwind `bg-blue-900` corresponde a um ElevatedButton nativo.
class SolidButton extends StatelessWidget {
  final SolidButtonProps props;
  const SolidButton(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return ButtonBase(props);
  }
}
