import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import '../../../molecules/breadcrumb/data/types.dart';
import '../../../molecules/breadcrumb/ui/Breadcrumbs.dart';

class AdminHeader extends StatelessWidget {
  const AdminHeader({
    super.key,
    required this.pageTitle,
    this.breadcrumb,
    this.addLabel,
    this.onAdd,
  });

  final String pageTitle;
  final BreadcrumbProps? breadcrumb;
  final String? addLabel;
  final VoidCallback? onAdd;

  @override
  Widget build(BuildContext context) {
    return Container(
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
                breadcrumb ??
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
              onPressed: onAdd,
              icon: const Icon(Icons.add, size: 16),
              label: Text(addLabel),
            ),
        ],
      ),
    );
  }
}
