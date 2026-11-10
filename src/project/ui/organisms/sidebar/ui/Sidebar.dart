import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';
import '../shared/sidebar-section/ui/SidebarSection.dart';
import '../shared/sidebar-profile/ui/SidebarProfile.dart';
import 'flutter/sidebar_rail.dart';

class Sidebar extends StatefulWidget {
  const Sidebar({
    super.key, this.variant = '01', this.theme = 'light', this.collapsed = false,
    this.brand = 'Untitled UI', this.workspace = 'workspace.untitledui.com',
    this.sections = const [], this.footerSections = const [],
    this.userName = 'Sienna Hewitt', this.userDetail = 'sienna@untitledui.com', this.avatarUrl,
  });

  final String variant, theme, brand, workspace, userName, userDetail;
  final bool collapsed;
  final List<SidebarSectionData> sections, footerSections;
  final String? avatarUrl;

  @override
  State<Sidebar> createState() => _SidebarState();
}

class _SidebarState extends State<Sidebar> {
  late final SidebarViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = SidebarViewModel(
      variant: widget.variant, theme: widget.theme, collapsed: widget.collapsed,
      brand: widget.brand, workspace: widget.workspace, sections: widget.sections,
      footerSections: widget.footerSections, userName: widget.userName,
      userDetail: widget.userDetail, avatarUrl: widget.avatarUrl,
    );
  }

  @override
  void didUpdateWidget(covariant Sidebar oldWidget) {
    super.didUpdateWidget(oldWidget);
    _vm.didUpdate(
      variant: widget.variant, theme: widget.theme, collapsed: widget.collapsed,
      brand: widget.brand, workspace: widget.workspace, sections: widget.sections,
      footerSections: widget.footerSections, userName: widget.userName,
      userDetail: widget.userDetail, avatarUrl: widget.avatarUrl,
    );
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) => Container(
        width: _vm.width,
        height: 720,
        decoration: BoxDecoration(color: _vm.backgroundColor, border: Border.all(color: _vm.borderColor), borderRadius: BorderRadius.circular(18)),
        clipBehavior: Clip.antiAlias,
        child: Row(children: [
          if (_vm.hasRail) SidebarRail(dark: _vm.isDark),
          Expanded(
            child: Padding(
              padding: const EdgeInsets.all(14),
              child: Column(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
                if (!_vm.hasRail)
                  Row(children: [
                    const Icon(Icons.hexagon_outlined),
                    if (!_vm.isCompact)
                      Padding(
                        padding: const EdgeInsets.only(left: 8),
                        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
                          Text(_vm.brand, style: const TextStyle(fontWeight: FontWeight.bold)),
                          if (_vm.showWorkspace) Text(_vm.workspace, style: const TextStyle(fontSize: 10)),
                        ]),
                      ),
                  ]),
                if (_vm.showSearch)
                  Padding(
                    padding: const EdgeInsets.symmetric(vertical: 12),
                    child: TextField(onChanged: _vm.updateSearch, decoration: InputDecoration(prefixIcon: const Icon(Icons.search, size: 18), hintText: _vm.searchHint, isDense: true, border: OutlineInputBorder(borderRadius: BorderRadius.circular(9)))),
                  ),
                Expanded(child: ListView(children: _vm.sections.map((s) => SidebarSection(section: s, compact: _vm.isCompact, dark: _vm.isDark)).toList())),
                ..._vm.footerSections.map((s) => SidebarSection(section: s, compact: _vm.isCompact, dark: _vm.isDark)),
                SidebarProfile(name: _vm.userName, detail: _vm.userDetail, avatarUrl: _vm.avatarUrl, compact: _vm.isCompact, dark: _vm.isDark),
              ]),
            ),
          ),
        ]),
      ),
    );
  }
}
