import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import '../data/types.dart';
import '../data/breadcrumbs_view_model.dart';

class Breadcrumbs extends StatefulWidget {
  final BreadcrumbsProps props;
  const Breadcrumbs(this.props, {super.key});

  @override
  State<Breadcrumbs> createState() => _BreadcrumbsState();
}

class _BreadcrumbsState extends State<Breadcrumbs> {
  late final BreadcrumbsViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = BreadcrumbsViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant Breadcrumbs oldWidget) {
    super.didUpdateWidget(oldWidget);
    _vm.didUpdate(widget.props);
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final primary = Theme.of(context).colorScheme.primary;
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) => Container(
        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
        decoration: NegoThemeData.surfaceDecoration(),
        child: Wrap(
          spacing: 6,
          children: List.generate(_vm.items.length, (idx) {
            final item = _vm.items[idx];
            final isLast = idx == _vm.items.length - 1 || item.active;
            return Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                if (idx > 0)
                  const Padding(
                    padding: EdgeInsets.only(right: 6),
                    child: Text('/', style: TextStyle(fontSize: 12, color: NegoThemeData.textMuted)),
                  ),
                InkWell(
                  onTap: isLast ? null : () => _vm.selectIndex(idx),
                  child: Text(
                    item.label,
                    style: TextStyle(
                      fontSize: 12,
                      color: isLast ? NegoThemeData.textPrimary : primary,
                    ),
                  ),
                ),
              ],
            );
          }),
        ),
      ),
    );
  }
}
