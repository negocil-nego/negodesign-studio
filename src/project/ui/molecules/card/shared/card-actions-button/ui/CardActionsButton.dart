import 'package:flutter/material.dart';
import '../data/types.dart';
import 'flutter/card_actions_variants.dart';

enum CardActionsButtonVariant { defaultVariant, iconOutline, mixed }

class CardActionsButton extends StatelessWidget {
  final CardActionsButtonProps props;
  final CardActionsButtonVariant variant;

  const CardActionsButton(
    this.props, {
    super.key,
    this.variant = CardActionsButtonVariant.defaultVariant,
  });

  @override
  Widget build(BuildContext context) {
    switch (variant) {
      case CardActionsButtonVariant.iconOutline:
        return CardActionsOutlineVariant(props: props, roundIconOnly: true);
      case CardActionsButtonVariant.mixed:
        return CardActionsMixedVariant(props: props);
      case CardActionsButtonVariant.defaultVariant:
        return CardActionsOutlineVariant(props: props);
    }
  }
}
