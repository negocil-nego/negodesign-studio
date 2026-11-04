import 'package:flutter/material.dart';

import '../data/types.dart';
import '../shared/sidebar-section/ui/SidebarSection.dart';
import '../shared/sidebar-profile/ui/SidebarProfile.dart';

class Sidebar extends StatelessWidget {
  const Sidebar({
    super.key,
    this.variant = '01',
    this.theme = 'light',
    this.collapsed = false,
    this.brand = 'Untitled UI',
    this.workspace = 'workspace.untitledui.com',
    this.sections = const [],
    this.footerSections = const [],
    this.userName = 'Sienna Hewitt',
    this.userDetail = 'sienna@untitledui.com',
    this.avatarUrl,
  });
  final String variant, theme, brand, workspace, userName, userDetail;
  final bool collapsed;
  final List<SidebarSectionData> sections, footerSections;
  final String? avatarUrl;
  @override
  Widget build(BuildContext context) {
    final dark = theme == 'dark', rail = variant == '02', compact = collapsed;
    final bg = dark ? const Color(0xFF09090B) : Colors.white;
    return Container(
      width: rail
          ? 420
          : compact
          ? 76
          : 280,
      height: 720,
      decoration: BoxDecoration(
        color: bg,
        border: Border.all(color: dark ? Colors.white12 : Colors.black12),
        borderRadius: BorderRadius.circular(18),
      ),
      clipBehavior: Clip.antiAlias,
      child: Row(
        children: [
          if (rail)
            Container(
              width: 68,
              color: dark ? const Color(0xFF18181B) : const Color(0xFFFAFAFA),
              child: Column(
                children: [
                  const SizedBox(height: 12),
                  const Icon(Icons.all_inclusive, color: Colors.blue),
                  const SizedBox(height: 12),
                  ...const [
                    Icons.grid_view,
                    Icons.search,
                    Icons.star,
                    Icons.home,
                    Icons.mail_outline,
                    Icons.person_outline,
                    Icons.settings_outlined,
                  ].map(
                    (icon) =>
                        IconButton(onPressed: null, icon: Icon(icon, size: 18)),
                  ),
                  const Spacer(),
                  const Icon(Icons.settings, size: 18),
                  const SizedBox(height: 12),
                ],
              ),
            ),
          Expanded(
            child: Padding(
              padding: const EdgeInsets.all(14),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  if (!rail)
                    Row(
                      children: [
                        const Icon(Icons.hexagon_outlined),
                        if (!compact)
                          Padding(
                            padding: const EdgeInsets.only(left: 8),
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text(
                                  brand,
                                  style: const TextStyle(
                                    fontWeight: FontWeight.bold,
                                  ),
                                ),
                                if (variant == '04')
                                  Text(
                                    workspace,
                                    style: const TextStyle(fontSize: 10),
                                  ),
                              ],
                            ),
                          ),
                      ],
                    ),
                  if (variant == '01' || variant == '02')
                    Padding(
                      padding: const EdgeInsets.symmetric(vertical: 12),
                      child: TextField(
                        decoration: InputDecoration(
                          prefixIcon: const Icon(Icons.search, size: 18),
                          hintText: compact ? '' : 'Pesquisar',
                          isDense: true,
                          border: OutlineInputBorder(
                            borderRadius: BorderRadius.circular(9),
                          ),
                        ),
                      ),
                    ),
                  Expanded(
                    child: ListView(
                      children: sections
                          .map(
                            (section) => SidebarSection(
                              section: section,
                              compact: compact,
                              dark: dark,
                            ),
                          )
                          .toList(),
                    ),
                  ),
                  if (footerSections.isNotEmpty)
                    ...footerSections.map(
                      (section) => SidebarSection(
                        section: section,
                        compact: compact,
                        dark: dark,
                      ),
                    ),
                  SidebarProfile(
                    name: userName,
                    detail: userDetail,
                    avatarUrl: avatarUrl,
                    compact: compact,
                    dark: dark,
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
