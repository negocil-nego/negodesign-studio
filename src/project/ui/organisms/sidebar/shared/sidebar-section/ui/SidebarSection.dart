import 'package:flutter/material.dart';

import '../../../data/types.dart';
import '../../sidebar-item/ui/SidebarItem.dart';

class SidebarSection extends StatelessWidget {
  const SidebarSection({
    super.key,
    required this.section,
    this.compact = false,
    this.dark = false,
  });
  final SidebarSectionData section;
  final bool compact, dark;
  @override
  Widget build(BuildContext context) => Column(
    crossAxisAlignment: CrossAxisAlignment.stretch,
    children: [
      if (section.title != null && !compact)
        Padding(
          padding: const EdgeInsets.fromLTRB(12, 16, 12, 6),
          child: Text(
            section.title!.toUpperCase(),
            style: TextStyle(
              fontSize: 10,
              color: dark ? Colors.white38 : Colors.black38,
              fontWeight: FontWeight.w600,
            ),
          ),
        ),
      ...section.items.map(
        (item) => SidebarItem(item: item, compact: compact, dark: dark),
      ),
    ],
  );
}
