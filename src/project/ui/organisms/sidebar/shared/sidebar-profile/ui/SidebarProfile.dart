import 'package:flutter/material.dart';

class SidebarProfile extends StatelessWidget {
  const SidebarProfile({
    super.key,
    required this.name,
    this.detail,
    this.avatarUrl,
    this.compact = false,
    this.dark = false,
  });
  final String name;
  final String? detail, avatarUrl;
  final bool compact, dark;
  @override
  Widget build(BuildContext context) => Container(
    padding: const EdgeInsets.all(8),
    decoration: BoxDecoration(
      color: dark ? Colors.white10 : Colors.white,
      border: Border.all(color: dark ? Colors.white12 : Colors.black12),
      borderRadius: BorderRadius.circular(12),
    ),
    child: Row(
      children: [
        CircleAvatar(
          radius: 18,
          backgroundImage: avatarUrl == null ? null : NetworkImage(avatarUrl!),
          child: avatarUrl == null ? const Icon(Icons.person) : null,
        ),
        if (!compact)
          Expanded(
            child: Padding(
              padding: const EdgeInsets.only(left: 8),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    name,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                      color: dark ? Colors.white : Colors.black,
                      fontSize: 12,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  if (detail != null)
                    Text(
                      detail!,
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        color: dark ? Colors.white54 : Colors.black54,
                        fontSize: 10,
                      ),
                    ),
                ],
              ),
            ),
          ),
      ],
    ),
  );
}
