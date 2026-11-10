import 'package:flutter/foundation.dart';

class BreadcrumbItem {
  final String label;
  final String? href;
  final bool active;

  const BreadcrumbItem({
    required this.label,
    this.href,
    this.active = false,
  });
}

class BreadcrumbsProps {
  final List<BreadcrumbItem> items;
  final ValueChanged<int>? onSelect;

  const BreadcrumbsProps({
    required this.items,
    this.onSelect,
  });
}
