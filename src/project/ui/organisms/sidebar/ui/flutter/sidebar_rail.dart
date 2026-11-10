import 'package:flutter/material.dart';

class SidebarRail extends StatelessWidget {
  final bool dark;
  const SidebarRail({super.key, required this.dark});

  static const _icons = [
    Icons.grid_view,
    Icons.search,
    Icons.star,
    Icons.home,
    Icons.mail_outline,
    Icons.person_outline,
    Icons.settings_outlined,
  ];

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 68,
      color: dark ? const Color(0xFF18181B) : const Color(0xFFFAFAFA),
      child: Column(
        children: [
          const SizedBox(height: 12),
          const Icon(Icons.all_inclusive, color: Colors.blue),
          const SizedBox(height: 12),
          ..._icons.map((icon) => IconButton(onPressed: null, icon: Icon(icon, size: 18))),
          const Spacer(),
          const Icon(Icons.settings, size: 18),
          const SizedBox(height: 12),
        ],
      ),
    );
  }
}

