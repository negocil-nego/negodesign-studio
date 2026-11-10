import 'package:flutter/material.dart';
import '../../../../data/theme/dart/theme_data.dart';
import '../data/types.dart';
import '../data/list_groups_view_model.dart';

class ListGroups extends StatefulWidget {
  final ListGroupsProps props;
  const ListGroups(this.props, {super.key});

  @override
  State<ListGroups> createState() => _ListGroupsState();
}

class _ListGroupsState extends State<ListGroups> {
  late final ListGroupsViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = ListGroupsViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant ListGroups oldWidget) {
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
        decoration: NegoThemeData.cardDecoration(radius: NegoThemeData.radiusLg),
        clipBehavior: Clip.antiAlias,
        child: Column(
          children: _vm.props.items.map((item) {
            final active = _vm.isItemActive(item);
            return InkWell(
              onTap: item.disabled ? null : () => _vm.select(item.id),
              child: Container(
                padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                color: active ? primary : NegoThemeData.blenderCard,
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Expanded(
                      child: Text(
                        item.label,
                        style: TextStyle(
                          fontSize: 13,
                          color: active ? Colors.white : NegoThemeData.textPrimary,
                        ),
                      ),
                    ),
                    if (item.badge != null)
                      Container(
                        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                        decoration: BoxDecoration(
                          color: active ? Colors.white24 : primary,
                          borderRadius: BorderRadius.circular(999),
                        ),
                        child: Text(
                          item.badge!,
                          style: const TextStyle(fontSize: 11, color: Colors.white),
                        ),
                      ),
                  ],
                ),
              ),
            );
          }).toList(),
        ),
      ),
    );
  }
}
