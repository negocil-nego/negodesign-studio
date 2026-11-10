import 'package:flutter/material.dart';
import '../../data/types.dart';

IconData resolveCardActionIcon(String? name) {
  switch (name) {
    case 'Cart': return Icons.shopping_cart_outlined;
    case 'Favorite': return Icons.favorite_border;
    case 'Share': return Icons.share_outlined;
    case 'Delete': return Icons.delete_outline;
    case 'Edit': return Icons.edit_outlined;
    case 'Add': return Icons.add;
    case 'Close': return Icons.close;
    case 'Message':
    case 'chat_bubble_outline': return Icons.chat_bubble_outline;
    default: return Icons.circle_outlined;
  }
}

class CardActionsOutlineVariant extends StatelessWidget {
  final CardActionsButtonProps props;
  final bool roundIconOnly;

  const CardActionsOutlineVariant({super.key, required this.props, this.roundIconOnly = false});

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: roundIconOnly ? MainAxisSize.min : MainAxisSize.max,
      children: [
        for (var i = 0; i < props.actions.length; i++) ...[
          if (i > 0) SizedBox(width: roundIconOnly ? 8 : 12),
          roundIconOnly ? _buildButton(props.actions[i]) : Expanded(child: _buildButton(props.actions[i])),
        ],
      ],
    );
  }

  Widget _buildButton(CardActionItem action) {
    return OutlinedButton(
      onPressed: action.onClick,
      style: OutlinedButton.styleFrom(
        padding: roundIconOnly ? const EdgeInsets.symmetric(horizontal: 12, vertical: 6) : const EdgeInsets.symmetric(vertical: 10),
        foregroundColor: roundIconOnly ? const Color(0xFF3F3F46) : const Color(0xFF111827),
        backgroundColor: roundIconOnly ? Colors.white : null,
        side: BorderSide(color: roundIconOnly ? const Color(0xFFD4D4D8) : const Color(0xFFE5E7EB)),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(roundIconOnly ? 999 : 16)),
      ),
      child: Row(mainAxisSize: MainAxisSize.min, mainAxisAlignment: MainAxisAlignment.center, children: [
        if (action.icon != null) ...[
          Icon(resolveCardActionIcon(action.icon), size: roundIconOnly ? 16 : 18),
          if (action.label != null) SizedBox(width: roundIconOnly ? 6 : 8),
        ],
        if (action.label != null)
          Text(action.label!, style: TextStyle(fontSize: roundIconOnly ? 12 : 14, fontWeight: roundIconOnly ? FontWeight.w500 : FontWeight.w600)),
      ]),
    );
  }
}

class CardActionsMixedVariant extends StatelessWidget {
  final CardActionsButtonProps props;
  const CardActionsMixedVariant({super.key, required this.props});

  @override
  Widget build(BuildContext context) {
    return Row(mainAxisSize: MainAxisSize.min, children: [
      for (var i = 0; i < props.actions.length; i++) ...[
        if (i > 0) const SizedBox(width: 8),
        if (i == 0)
          ElevatedButton(
            onPressed: props.actions[i].onClick,
            style: ElevatedButton.styleFrom(
              padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 8),
              backgroundColor: const Color(0xFF18181B),
              foregroundColor: Colors.white,
              elevation: 0,
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(999)),
            ),
            child: Row(mainAxisSize: MainAxisSize.min, children: [
              if (props.actions[i].icon != null) ...[Icon(resolveCardActionIcon(props.actions[i].icon), size: 16), const SizedBox(width: 6)],
              if (props.actions[i].label != null) Text(props.actions[i].label!, style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 14)),
            ]),
          )
        else
          SizedBox(
            width: 36,
            height: 36,
            child: OutlinedButton(
              onPressed: props.actions[i].onClick,
              style: OutlinedButton.styleFrom(padding: EdgeInsets.zero, foregroundColor: const Color(0xFF3F3F46), backgroundColor: Colors.white, side: const BorderSide(color: Color(0xFFD4D4D8)), shape: const CircleBorder()),
              child: Icon(resolveCardActionIcon(props.actions[i].icon), size: 18),
            ),
          ),
      ],
    ]);
  }
}
