import 'package:flutter/material.dart';
import '../type/types.dart';
import '../../../shared/base/ui/ButtonBase.dart';
import '../../solid/ui/SolidButton.dart';
import '../../outline/ui/OutlineButton.dart';
import '../../solid/data/types.dart';
import '../../outline/data/types.dart';

/// Aproximação em Flutter do Button: escolhe a variante em runtime.
class Button extends StatelessWidget {
  final ButtonProps props;
  const Button(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    if (props.variant == 'outline') {
      return OutlineButton(OutlineButtonProps(
        icon: props.icon,
        text: props.text,
        className: props.className,
        isLoading: props.isLoading,
        isSkeleton: props.isSkeleton,
        isDisabled: props.isDisabled,
        onClick: props.onClick,
      ));
    }
    if (props.variant == 'solid') {
      return SolidButton(SolidButtonProps(
        icon: props.icon,
        text: props.text,
        className: props.className,
        isLoading: props.isLoading,
        isSkeleton: props.isSkeleton,
        isDisabled: props.isDisabled,
        onClick: props.onClick,
      ));
    }
    return ButtonBase(props);
  }
}
