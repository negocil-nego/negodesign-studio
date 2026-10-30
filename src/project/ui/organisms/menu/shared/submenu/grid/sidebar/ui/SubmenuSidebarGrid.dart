import 'package:flutter/material.dart';
import '../data/types.dart';
import '..\..\..\..\menu\menu-title\ui\MenuTitle.dart';
import '..\..\..\simple\title\ui\SubmenuTitle.dart';

class SubmenuSidebarGrid extends StatefulWidget {
  final SubmenuSidebarGridProps props;
  const SubmenuSidebarGrid(this.props, {super.key});
  @override
  State<SubmenuSidebarGrid> createState() => _SubmenuSidebarGridState();
}
class _SubmenuSidebarGridState extends State<SubmenuSidebarGrid> {
  late int selected;
  @override
  void initState() { super.initState(); selected = widget.props.defaultSelected.clamp(0, (widget.props.items.length - 1).clamp(0, 999)); }
  @override
  Widget build(BuildContext context) {
    final items = widget.props.items;
    return Row(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
      SizedBox(width: 220, child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [const Padding(padding: EdgeInsets.all(16), child: Text('NAVEGAÇÃO')), for (var i = 0; i < items.length; i++) Padding(padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2), child: TextButton(onPressed: () => setState(() => selected = i), child: Align(alignment: Alignment.centerLeft, child: Text(items[i].title?.textProps?.Text ?? 'Secção ${i + 1}'))))])),
      const VerticalDivider(width: 1),
      Expanded(child: Padding(padding: const EdgeInsets.all(24), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [if (widget.props.title != null) MenuTitle(widget.props.title!), const SizedBox(height: 16), if (items.isEmpty) const Text('Sem secções para apresentar.') else SubmenuTitle(items[selected])]))),
    ]);
  }
}
