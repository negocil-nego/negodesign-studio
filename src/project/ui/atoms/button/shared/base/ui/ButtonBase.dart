import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../../icon/ui/Icon.dart' as atom;

/// Aproximação em Flutter do ButtonBase.
/// As classes Tailwind de `className` não têm equivalente directo, pelo que
/// usamos um ElevatedButton nativo com o ícone e/ou o texto indicados.
class ButtonBase extends StatelessWidget {
  final ButtonBaseProps props;
  const ButtonBase(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: (props.isDisabled || props.isLoading) ? null : props.onClick,
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          if (props.icon != null) atom.Icon(props.icon!),
          if (props.text != null) Text(props.text!),
        ],
      ),
    );
  }
}
