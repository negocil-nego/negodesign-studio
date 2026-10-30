import 'package:flutter/material.dart';
import '../data/types.dart';
import '../../../../../../atoms/icon/ui/icon.dart' as iconui;
import '../../../../../../atoms/text/ui/text.dart' as textui;

class MenuText extends StatelessWidget {
  final MenuTextProps props;
  const MenuText(this.props, {super.key});
  @override
  Widget build(BuildContext context) => Row(mainAxisSize: MainAxisSize.min, children: [
    if (props.icon != null) iconui.Icon(props.icon!),
    if (props.textProps != null) textui.Text(props.textProps!),
  ]);
}
