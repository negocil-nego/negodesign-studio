import 'package:flutter/foundation.dart';

class ListGroupEntry {
  final String id;
  final String label;
  final String? badge;
  final String? description;
  final String? meta;
  final String? footer;
  final bool active;
  final bool disabled;

  const ListGroupEntry({
    required this.id,
    required this.label,
    this.badge,
    this.description,
    this.meta,
    this.footer,
    this.active = false,
    this.disabled = false,
  });
}

class ListGroupsProps {
  final List<ListGroupEntry> items;
  final String? activeId;
  final ValueChanged<String>? onSelect;

  const ListGroupsProps({
    required this.items,
    this.activeId,
    this.onSelect,
  });
}
