import 'package:flutter/material.dart';
import '../data/types.dart';
import '..\..\..\..\menu\menu-grid\ui\MenuText.dart';
import '..\..\..\..\menu\menu-title\ui\MenuTitle.dart';

class SubmenuTitle extends StatelessWidget {
  final SubmenuTitleProps props;
  const SubmenuTitle(this.props, {super.key});
  @override
  Widget build(BuildContext context) => Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
    if (props.title != null) MenuGridText(props.title!),
    if (props.items != null) Wrap(spacing: 20, runSpacing: 20, children: props.items!.map((item) => SizedBox(width: 220, child: MenuTitle(item))).toList()),
  ]);
}
