import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import '../../../molecules/breadcrumb/data/types.dart';
import '../../../molecules/breadcrumb/ui/Breadcrumbs.dart';
import '../../../organisms/data-table/ui/DataTable.dart';
import '../../../organisms/menu/simple/ui/Menu.dart';
import '../../../organisms/modal/form/ui/ModalForm.dart';
import '../../../organisms/sidebar/ui/Sidebar.dart';
import '../data/types.dart';

class AdminCrud extends StatefulWidget {
  const AdminCrud({super.key, required this.props});
  final AdminProps props;

  @override
  State<AdminCrud> createState() => _AdminCrudState();
}

class _AdminCrudState extends State<AdminCrud> {
  bool _modalOpen = false;
  String _modalMode = 'create';

  void _openCreate() {
    setState(() {
      _modalMode = 'create';
      _modalOpen = true;
    });
    widget.props.datatableProps?.onAdd?.call();
  }

  void _closeModal() {
    setState(() => _modalOpen = false);
    widget.props.modalFormProps?.onClose?.call();
  }

  @override
  Widget build(BuildContext context) {
    final p = widget.props;
    final pageTitle = p.title ?? p.datatableProps?.title ?? p.modalFormProps?.title ?? 'Team List';
    final addLabel = p.addButtonLabel ?? p.datatableProps?.addLabel ?? (p.modalFormProps != null ? 'ADD NEW' : null);
    final isOpen = (p.modalFormProps?.open ?? false) || _modalOpen;

    return Stack(
      children: [
        Container(
          color: NegoThemeData.blenderBase,
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              if (p.sidebarProps != null)
                Sidebar(
                  variant: p.sidebarProps!.variant,
                  theme: p.sidebarProps!.theme,
                  collapsed: p.sidebarProps!.collapsed,
                  brand: p.sidebarProps!.brand,
                  workspace: p.sidebarProps!.workspace,
                  sections: p.sidebarProps!.sections,
                  footerSections: p.sidebarProps!.footerSections,
                  userName: p.sidebarProps!.userName,
                  userDetail: p.sidebarProps!.userDetail,
                  avatarUrl: p.sidebarProps!.avatarUrl,
                ),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    if (p.menuProps != null) Menu(p.menuProps!),
                    Container(
                      padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
                      decoration: const BoxDecoration(
                        color: NegoThemeData.blenderCard,
                        border: Border(bottom: BorderSide(color: NegoThemeData.blenderBorder)),
                      ),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                pageTitle,
                                style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold, color: NegoThemeData.textPrimary),
                              ),
                              const SizedBox(height: 4),
                              Breadcrumbs(
                                p.breadcrumb ??
                                    BreadcrumbsProps(
                                      items: [
                                        const BreadcrumbItem(label: 'Admin Dashboard', href: '#admin'),
                                        BreadcrumbItem(label: pageTitle, active: true),
                                      ],
                                    ),
                              ),
                            ],
                          ),
                          if (addLabel != null)
                            ElevatedButton.icon(
                              onPressed: _openCreate,
                              icon: const Icon(Icons.add, size: 16),
                              label: Text(addLabel),
                            ),
                        ],
                      ),
                    ),
                    if (p.datatableProps != null)
                      Expanded(
                        child: SingleChildScrollView(
                          padding: const EdgeInsets.all(24),
                          child: DataTableWidget(
                            variant: p.datatableProps!.variant,
                            title: p.datatableProps!.title,
                            columns: p.datatableProps!.columns,
                            rows: p.datatableProps!.rows,
                            actions: p.datatableProps!.actions,
                            addLabel: p.datatableProps!.addLabel,
                            onAdd: _openCreate,
                            onRowAction: (action, ri, row) {
                              if (action.toLowerCase() == 'edit' || action.toLowerCase() == 'editar') {
                                setState(() {
                                  _modalMode = 'edit';
                                  _modalOpen = true;
                                });
                              }
                              p.datatableProps!.onRowAction?.call(action, ri, row);
                            },
                          ),
                        ),
                      ),
                  ],
                ),
              ),
            ],
          ),
        ),
        if (p.modalFormProps != null && isOpen)
          ModalForm(
            open: isOpen,
            title: p.modalFormProps!.title.isNotEmpty
                ? p.modalFormProps!.title
                : (_modalMode == 'edit' ? 'Edit $pageTitle' : 'Create $pageTitle'),
            form: p.modalFormProps!.form,
            onClose: _closeModal,
          ),
      ],
    );
  }
}

