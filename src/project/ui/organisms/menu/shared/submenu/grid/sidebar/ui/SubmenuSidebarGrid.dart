import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';
import '../../../../menu/menu-title/ui/MenuTitle.dart';
import '../../../simple/title/ui/SubmenuTitle.dart';

class SubmenuSidebarGrid extends StatefulWidget {
  final SubmenuSidebarGridProps props;
  const SubmenuSidebarGrid(this.props, {super.key});

  @override
  State<SubmenuSidebarGrid> createState() => _SubmenuSidebarGridState();
}

class _SubmenuSidebarGridState extends State<SubmenuSidebarGrid> {
  late final SubmenuSidebarGridViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = SubmenuSidebarGridViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant SubmenuSidebarGrid oldWidget) {
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
    final items = widget.props.items;
    return ListenableBuilder(
      listenable: _vm,
      builder: (context, _) => Row(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
        SizedBox(
          width: 220,
          child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
            const Padding(padding: EdgeInsets.all(16), child: Text('NAVIGATION')),
            for (var i = 0; i < items.length; i++)
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                child: TextButton(
                  onPressed: () => _vm.selectIndex(i),
                  child: Align(alignment: Alignment.centerLeft, child: Text(items[i].title?.textProps?.Text ?? 'Section ${i + 1}')),
                ),
              ),
          ]),
        ),
        const VerticalDivider(width: 1),
        Expanded(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              if (widget.props.title != null) MenuTitle(widget.props.title!),
              const SizedBox(height: 16),
              if (items.isEmpty) const Text('No sections to display.') else SubmenuTitle(items[_vm.selected]),
            ]),
          ),
        ),
      ]),
    );
  }
}
