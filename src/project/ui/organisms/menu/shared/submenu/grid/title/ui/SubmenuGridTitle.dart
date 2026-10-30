import 'package:flutter/material.dart';
import '../data/types.dart';
import '..\..\..\..\menu\menu-title\ui\MenuTitle.dart';
import '..\..\..\simple\title\ui\SubmenuTitle.dart';

class SubmenuGridTitle extends StatelessWidget {
  final SubmenuGridTitleProps props;
  const SubmenuGridTitle(this.props, {super.key});
  @override
  Widget build(BuildContext context) => Row(crossAxisAlignment: CrossAxisAlignment.start, children: [
    if (props.title != null) SizedBox(width: 260, child: Padding(padding: const EdgeInsets.all(24), child: MenuTitle(props.title!))),
    if (props.items != null) Expanded(child: Wrap(spacing: 24, runSpacing: 24, children: props.items!.map((item) => SizedBox(width: 220, child: SubmenuTitle(item))).toList())),
  ]);
}
