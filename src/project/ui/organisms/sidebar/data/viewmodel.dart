import 'package:flutter/material.dart';
import 'types.dart';

class SidebarViewModel extends ChangeNotifier {
  String variant;
  String theme;
  bool collapsed;
  String brand;
  String workspace;
  List<SidebarSectionData> sections;
  List<SidebarSectionData> footerSections;
  String userName;
  String userDetail;
  String? avatarUrl;

  String searchQuery = '';

  SidebarViewModel({
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

  void didUpdate({
    required String variant,
    required String theme,
    required bool collapsed,
    required String brand,
    required String workspace,
    required List<SidebarSectionData> sections,
    required List<SidebarSectionData> footerSections,
    required String userName,
    required String userDetail,
    String? avatarUrl,
  }) {
    this.variant = variant;
    this.theme = theme;
    this.collapsed = collapsed;
    this.brand = brand;
    this.workspace = workspace;
    this.sections = sections;
    this.footerSections = footerSections;
    this.userName = userName;
    this.userDetail = userDetail;
    this.avatarUrl = avatarUrl;
  }

  bool get isDark => theme == 'dark';
  bool get hasRail => variant == '02';
  bool get isCompact => collapsed;
  bool get showSearch => variant == '01' || variant == '02';
  bool get showWorkspace => variant == '04';

  double get width => hasRail ? 420 : (isCompact ? 76 : 280);
  Color get backgroundColor => isDark ? const Color(0xFF09090B) : Colors.white;
  Color get borderColor => isDark ? Colors.white12 : Colors.black12;
  String get searchHint => isCompact ? '' : 'Search';

  void updateSearch(String value) {
    searchQuery = value;
    notifyListeners();
  }
}
