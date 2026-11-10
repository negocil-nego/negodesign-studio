import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import '../data/types.dart';
import '../data/pagination_nav_view_model.dart';

class PaginationNav extends StatefulWidget {
  final PaginationNavProps props;
  const PaginationNav(this.props, {super.key});

  @override
  State<PaginationNav> createState() => _PaginationNavState();
}

class _PaginationNavState extends State<PaginationNav> {
  late final PaginationNavViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = PaginationNavViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant PaginationNav oldWidget) {
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
        decoration: NegoThemeData.surfaceDecoration(),
        clipBehavior: Clip.antiAlias,
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            _buildCell('«', false, () => _vm.goTo(_vm.current - 1), primary),
            ..._vm.pages.map(
              (p) => _buildCell('$p', p == _vm.current, () => _vm.goTo(p), primary),
            ),
            _buildCell('»', false, () => _vm.goTo(_vm.current + 1), primary),
          ],
        ),
      ),
    );
  }

  Widget _buildCell(String text, bool active, VoidCallback onTap, Color primary) {
    return InkWell(
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 7),
        color: active ? primary : NegoThemeData.blenderSurface,
        child: Text(
          text,
          style: TextStyle(
            fontSize: 12,
            color: active ? Colors.white : NegoThemeData.textSecondary,
          ),
        ),
      ),
    );
  }
}
