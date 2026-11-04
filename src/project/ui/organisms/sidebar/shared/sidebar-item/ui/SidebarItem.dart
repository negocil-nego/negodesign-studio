import 'package:flutter/material.dart';

import '../../../data/types.dart';

class SidebarItem extends StatelessWidget {
  const SidebarItem({
    super.key,
    required this.item,
    this.compact = false,
    this.dark = false,
  });
  final SidebarItemData item;
  final bool compact, dark;
  @override
  Widget build(BuildContext context) => ListTile(
    dense: true,
    leading: Icon(item.icon, size: 18),
    title: compact
        ? null
        : Text(
            item.label,
            overflow: TextOverflow.ellipsis,
            style: const TextStyle(fontSize: 13),
          ),
    trailing: compact
        ? null
        : item.badge == null
        ? null
        : Text(item.badge!, style: const TextStyle(fontSize: 10)),
    selected: item.active,
    selectedTileColor: dark ? Colors.white12 : const Color(0xFFEFF6FF),
    onTap: () {},
    contentPadding: EdgeInsets.symmetric(horizontal: compact ? 18 : 12),
    shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(9)),
  );
}
