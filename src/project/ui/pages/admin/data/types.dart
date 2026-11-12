import 'package:flutter/foundation.dart';
import '../../../molecules/breadcrumb/data/types.dart';
import '../../../organisms/data-table/data/types.dart';
import '../../../organisms/menu/simple/data/types.dart';
import '../../../organisms/sidebar/data/types.dart';
import '../../../molecules/form/data/types.dart';

class AdminModalFormConfig {
  const AdminModalFormConfig({
    this.open = false,
    this.title = '',
    this.form,
    this.onClose,
  });

  final bool open;
  final String title;
  final FormProps? form;
  final VoidCallback? onClose;
}

class AdminDataTableConfig {
  const AdminDataTableConfig({
    this.variant = '01',
    this.title,
    this.columns,
    this.rows,
    this.actions,
    this.addLabel,
    this.onAdd,
    this.onRowAction,
  });

  final String variant;
  final String? title;
  final List<TextSegment>? columns;
  final List<TextSegment>? rows;
  final List<String>? actions;
  final String? addLabel;
  final VoidCallback? onAdd;
  final void Function(String action, int rowIndex, List<TextSegment> row)? onRowAction;
}

class AdminSidebarConfig {
  const AdminSidebarConfig({
    this.variant = '01',
    this.theme = 'dark',
    this.collapsed = false,
    this.brand = 'Admin Workspace',
    this.workspace = 'admin.negodesign.studio',
    this.sections = const [],
    this.footerSections = const [],
    this.userName = 'Admin User',
    this.userDetail = 'admin@negodesign.com',
    this.avatarUrl,
  });

  final String variant;
  final String theme;
  final bool collapsed;
  final String brand;
  final String workspace;
  final List<SidebarSectionData> sections;
  final List<SidebarSectionData> footerSections;
  final String userName;
  final String userDetail;
  final String? avatarUrl;
}

class AdminProps {
  const AdminProps({
    this.datatableProps,
    this.modalFormProps,
    this.sidebarProps,
    this.menuProps,
    this.breadcrumb,
    this.title,
    this.addButtonLabel,
  });

  final AdminDataTableConfig? datatableProps;
  final AdminModalFormConfig? modalFormProps;
  final AdminSidebarConfig? sidebarProps;
  final MenuProps? menuProps;
  final BreadcrumbProps? breadcrumb;
  final String? title;
  final String? addButtonLabel;
}

typedef AdminCrudProps = AdminProps;

