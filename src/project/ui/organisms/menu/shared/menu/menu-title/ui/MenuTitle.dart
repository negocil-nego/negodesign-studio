import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../../../../atoms/text/ui/text.dart' as textui;

class MenuTitle extends StatelessWidget {
  final MenuTitleProps props;
  const MenuTitle(this.props, {super.key});
  @override
  Widget build(BuildContext context) => Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
    if (props.title != null) props.href != null ? InkWell(onTap: () {}, child: textui.Text(props.title!)) : textui.Text(props.title!),
    if (props.description != null) Padding(padding: const EdgeInsets.only(top: 8), child: textui.Text(props.description!)),
    if (props.link != null) Padding(padding: const EdgeInsets.only(top: 12), child: Row(mainAxisSize: MainAxisSize.min, children: [textui.Text(props.link!), const Text(' →')])),
  ]);
}
