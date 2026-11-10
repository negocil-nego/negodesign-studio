import 'package:flutter/material.dart';
import '../../data/types.dart';
import '../../data/viewmodel.dart';
import '../../../shared/card-actions-button/ui/CardActionsButton.dart';
import '../../../shared/card-description/ui/CardDescription.dart';
import '../../../shared/card-logo/ui/CardLogo.dart';
import '../../../shared/card-tags/ui/CardTags.dart';
import '../../../shared/card-title/ui/CardTitle.dart';

class CardProfile03 extends StatelessWidget {
  final CardProfileBaseProps props;
  const CardProfile03(this.props, {super.key});

  @override
  Widget build(BuildContext context) {
    final vm = CardProfileViewModel(props);
    return Container(
      width: 352,
      clipBehavior: Clip.antiAlias,
      decoration: BoxDecoration(
        color: Theme.of(context).cardColor,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: const Color(0xFFE4E4E7)),
        boxShadow: const [BoxShadow(color: Color(0x0A000000), blurRadius: 4, offset: Offset(0, 1))],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        mainAxisSize: MainAxisSize.min,
        children: [
          SizedBox(
            height: 128,
            child: Stack(
              clipBehavior: Clip.none,
              children: [
                Positioned.fill(child: vm.buildMedia(gradientFallback: true)),
                if (props.editActionProps != null)
                  Positioned(
                    top: 12,
                    right: 12,
                    child: CardActionsButton(props.editActionProps!, variant: CardActionsButtonVariant.iconOutline),
                  ),
                if (props.logoProps != null)
                  Positioned(
                    bottom: -40,
                    left: 20,
                    child: Container(
                      width: 80,
                      height: 80,
                      clipBehavior: Clip.antiAlias,
                      decoration: BoxDecoration(
                        color: Colors.white,
                        shape: BoxShape.circle,
                        border: Border.all(color: Colors.white, width: 4),
                        boxShadow: const [BoxShadow(color: Color(0x26000000), blurRadius: 6)],
                      ),
                      child: CardLogo(props.logoProps!),
                    ),
                  ),
              ],
            ),
          ),
          Padding(
            padding: const EdgeInsets.fromLTRB(20, 48, 20, 20),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Expanded(
                      child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
                        if (props.titleProps != null) CardTitle(props.titleProps!),
                        if (props.handleProps != null) Text(props.handleProps!, style: const TextStyle(fontSize: 14, color: Color(0xFF71717A))),
                      ]),
                    ),
                    if (props.actionsProps != null) CardActionsButton(props.actionsProps!, variant: CardActionsButtonVariant.mixed),
                  ],
                ),
                const SizedBox(height: 16),
                if (vm.hasAboutSection) ...[
                  const Text('About', style: TextStyle(fontSize: 14, fontWeight: FontWeight.w600, color: Color(0xFF18181B))),
                  const SizedBox(height: 4),
                  if (props.descriptionProps != null) CardDescription(props.descriptionProps!),
                  const SizedBox(height: 16),
                ],
                if (props.tagsProps != null) CardTags(CardTagsProps(tags: props.tagsProps!.tags, variant: 'stats')),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
