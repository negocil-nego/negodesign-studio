import 'package:flutter/material.dart';
import '../../menu-text/data/types.dart';
import '../../../../../../atoms/icon/ui/icon.dart' as iconui;
import '../../../../../../atoms/text/ui/text.dart' as textui;

class MenuGridText extends StatelessWidget {
  final MenuTextProps props;
  const MenuGridText(this.props, {super.key});
  @override
  Widget build(BuildContext context) => Row(crossAxisAlignment: CrossAxisAlignment.start, children: [
    if (props.icon != null) Padding(padding: const EdgeInsets.only(right: 8), child: iconui.Icon(props.icon!)),
    if (props.textProps != null) Expanded(child: textui.Text(props.textProps!)),
  ]);
}
