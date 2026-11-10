import 'package:flutter/foundation.dart';

class UnderlineNavItem {
  final String value;
  final String label;
  final bool disabled;

  const UnderlineNavItem({
    required this.value,
    required this.label,
    this.disabled = false,
  });
}

class UnderlineNavProps {
  final List<UnderlineNavItem> items;
  final String value;
  final ValueChanged<String>? onValueChange;

  const UnderlineNavProps({
    required this.items,
    this.value = '',
    this.onValueChange,
  });
}
