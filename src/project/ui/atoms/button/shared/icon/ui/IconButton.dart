import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../variant/default/ui/Button.dart';
import '../../../variant/default/type/types.dart';

/// Aproximação em Flutter do IconButton: delega no Button padrão,
/// passando o ícone e o estado de carregamento.
class IconButton extends StatelessWidget {
  final IconButtonProps props;
  const IconButton(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return Button(ButtonProps(
      icon: props.icon,
      isSkeleton: props.isLoading,
      onClick: props.onClick,
    ));
  }
}
