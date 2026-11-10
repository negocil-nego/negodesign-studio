import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import '../data/types.dart';
import '../data/underline_nav_view_model.dart';

class UnderlineNav extends StatefulWidget {
  final UnderlineNavProps props;
  const UnderlineNav(this.props, {super.key});

  @override
  State<UnderlineNav> createState() => _UnderlineNavState();
}

class _UnderlineNavState extends State<UnderlineNav> {
  late final UnderlineNavViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = UnderlineNavViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant UnderlineNav oldWidget) {
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
        decoration: const BoxDecoration(
          border: Border(bottom: BorderSide(color: NegoThemeData.blenderBorder)),
        ),
        child: Wrap(
          spacing: 18,
          children: _vm.props.items.map((item) {
            final isActive = item.value == _vm.active;
            return InkWell(
              onTap: item.disabled ? null : () => _vm.select(item.value),
              child: Container(
                padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 4),
                decoration: BoxDecoration(
                  border: Border(
                    bottom: BorderSide(
                      color: isActive ? primary : Colors.transparent,
                      width: 2,
                    ),
                  ),
                ),
                child: Text(
                  item.label,
                  style: TextStyle(
                    fontSize: 13,
                    fontWeight: isActive ? FontWeight.w600 : FontWeight.w500,
                    color: item.disabled
                        ? NegoThemeData.textMuted
                        : (isActive ? primary : NegoThemeData.textSecondary),
                  ),
                ),
              ),
            );
          }).toList(),
        ),
      ),
    );
  }
}
