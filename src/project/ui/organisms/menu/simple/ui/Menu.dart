import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../../atoms/button/variant/default/ui/Button.dart';
import '../../../../atoms/image/ui/Image.dart' as imageui;
import '..\..\shared\submenu\grid\title\ui\SubmenuGridTitle.dart';
import '..\..\shared\submenu\grid\sidebar\ui\SubmenuSidebarGrid.dart';
import '../../shared/submenu/grid/title/data/types.dart';
import '../../shared/submenu/grid/sidebar/data/types.dart';

class Menu extends StatefulWidget {
  final MenuProps props;
  const Menu(this.props, {super.key});
  @override
  State<Menu> createState() => _MenuState();
}
class _MenuState extends State<Menu> {
  int? openIndex;
  @override
  void initState() { super.initState(); openIndex = widget.props.defaultOpenSubmenu; }
  @override
  Widget build(BuildContext context) {
    final menus = widget.props.menus;
    final active = openIndex != null && openIndex! < menus.length ? menus[openIndex!] : null;
    final hasPanel = active is SubmenuSidebarGridProps || active is SubmenuGridTitleProps;
    return Column(crossAxisAlignment: CrossAxisAlignment.stretch, children: [
      Container(padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12), decoration: BoxDecoration(border: Border(bottom: BorderSide(color: Theme.of(context).dividerColor))), child: Row(children: [
        if (widget.props.logo != null) imageui.Image(widget.props.logo!),
        const Spacer(),
        for (var i = 0; i < menus.length; i++) Padding(padding: const EdgeInsets.symmetric(horizontal: 6), child: TextButton(onPressed: () => setState(() => openIndex = openIndex == i ? null : i), child: Text(menus[i] is SubmenuSidebarGridProps ? (menus[i] as SubmenuSidebarGridProps).title?.title?.Text ?? 'Explorar' : menus[i] is SubmenuGridTitleProps ? (menus[i] as SubmenuGridTitleProps).label ?? 'Explorar' : 'Menu ${i + 1}'))),
        for (final button in widget.props.buttonsProps) Padding(padding: const EdgeInsets.only(left: 8), child: Button(button)),
      ])),
      if (hasPanel) Container(decoration: BoxDecoration(border: Border.all(color: Theme.of(context).dividerColor)), child: active is SubmenuSidebarGridProps ? SubmenuSidebarGrid(active) : SubmenuGridTitle(active as SubmenuGridTitleProps)),
    ]);
  }
}
