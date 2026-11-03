import 'package:flutter/material.dart';
import '../data/types.dart';

enum CardActionsButtonVariant { defaultVariant, iconOutline, mixed }

/// Flutter approximation of the CardActionsButton component.
/// Supports three rendering variants:
///   - [CardActionsButtonVariant.defaultVariant] — outline buttons with optional icon + label
///   - [CardActionsButtonVariant.iconOutline]    — icon-only round outline buttons (e.g. Edit Profile)
///   - [CardActionsButtonVariant.mixed]          — first action = solid pill (Follow), rest = icon-only outline (Message)
class CardActionsButton extends StatelessWidget {
  final CardActionsButtonProps props;
  final CardActionsButtonVariant variant;

  const CardActionsButton(
    this.props, {
    super.key,
    this.variant = CardActionsButtonVariant.defaultVariant,
  });

  /// Maps a semantic icon name to the Material equivalent.
  IconData _iconFor(String? name) {
    switch (name) {
      case 'Cart':
        return Icons.shopping_cart_outlined;
      case 'Favorite':
        return Icons.favorite_border;
      case 'Share':
        return Icons.share_outlined;
      case 'Delete':
        return Icons.delete_outline;
      case 'Edit':
        return Icons.edit_outlined;
      case 'Add':
        return Icons.add;
      case 'Close':
        return Icons.close;
      case 'Message':
      case 'chat_bubble_outline':
        return Icons.chat_bubble_outline;
      default:
        return Icons.circle_outlined;
    }
  }

  @override
  Widget build(BuildContext context) {
    switch (variant) {
      case CardActionsButtonVariant.iconOutline:
        return _buildIconOutline();
      case CardActionsButtonVariant.mixed:
        return _buildMixed();
      case CardActionsButtonVariant.defaultVariant:
      default:
        return _buildDefault();
    }
  }

  // ── default: outline buttons with icon + label ────────────────────
  Widget _buildDefault() {
    return Row(
      children: [
        for (var i = 0; i < props.actions.length; i++) ...[
          if (i > 0) const SizedBox(width: 12),
          Expanded(
            child: OutlinedButton(
              onPressed: props.actions[i].onClick,
              style: OutlinedButton.styleFrom(
                padding: const EdgeInsets.symmetric(vertical: 10),
                foregroundColor: const Color(0xFF111827),
                side: const BorderSide(color: Color(0xFFE5E7EB)),
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
              ),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  if (props.actions[i].icon != null) ...[
                    Icon(_iconFor(props.actions[i].icon), size: 18),
                    const SizedBox(width: 8),
                  ],
                  Text(
                    props.actions[i].label ?? '',
                    style: const TextStyle(fontWeight: FontWeight.w600),
                  ),
                ],
              ),
            ),
          ),
        ],
      ],
    );
  }

  // ── icon-outline: round outline button(s) with optional icon + label ─
  Widget _buildIconOutline() {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        for (var i = 0; i < props.actions.length; i++) ...[
          if (i > 0) const SizedBox(width: 8),
          OutlinedButton(
            onPressed: props.actions[i].onClick,
            style: OutlinedButton.styleFrom(
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
              foregroundColor: const Color(0xFF3F3F46),
              backgroundColor: Colors.white,
              side: const BorderSide(color: Color(0xFFD4D4D8)),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(999)),
            ),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                if (props.actions[i].icon != null) ...[
                  Icon(_iconFor(props.actions[i].icon), size: 16),
                  if (props.actions[i].label != null) const SizedBox(width: 6),
                ],
                if (props.actions[i].label != null)
                  Text(
                    props.actions[i].label!,
                    style: const TextStyle(fontSize: 12, fontWeight: FontWeight.w500),
                  ),
              ],
            ),
          ),
        ],
      ],
    );
  }

  // ── mixed: first = solid pill (Follow), rest = icon-only round outline ─
  Widget _buildMixed() {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        for (var i = 0; i < props.actions.length; i++) ...[
          if (i > 0) const SizedBox(width: 8),
          if (i == 0)
            // Solid pill — Follow
            ElevatedButton(
              onPressed: props.actions[i].onClick,
              style: ElevatedButton.styleFrom(
                padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 8),
                backgroundColor: const Color(0xFF18181B),
                foregroundColor: Colors.white,
                elevation: 0,
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(999)),
              ),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  if (props.actions[i].icon != null) ...[
                    Icon(_iconFor(props.actions[i].icon), size: 16),
                    const SizedBox(width: 6),
                  ],
                  if (props.actions[i].label != null)
                    Text(
                      props.actions[i].label!,
                      style: const TextStyle(fontWeight: FontWeight.w600, fontSize: 14),
                    ),
                ],
              ),
            )
          else
            // Icon-only round outline — Message
            SizedBox(
              width: 36,
              height: 36,
              child: OutlinedButton(
                onPressed: props.actions[i].onClick,
                style: OutlinedButton.styleFrom(
                  padding: EdgeInsets.zero,
                  foregroundColor: const Color(0xFF3F3F46),
                  backgroundColor: Colors.white,
                  side: const BorderSide(color: Color(0xFFD4D4D8)),
                  shape: const CircleBorder(),
                ),
                child: Icon(_iconFor(props.actions[i].icon), size: 18),
              ),
            ),
        ],
      ],
    );
  }
}
