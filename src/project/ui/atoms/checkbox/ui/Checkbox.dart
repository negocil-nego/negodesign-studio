import 'package:flutter/material.dart';
import '../data/types.dart';

class Checkbox extends StatefulWidget {
  final CheckboxProps props;
  const Checkbox(this.props, {super.key});

  @override
  State<Checkbox> createState() => _CheckboxState();
}

class _CheckboxState extends State<Checkbox> {
  late bool _checked;

  @override
  void initState() {
    super.initState();
    _checked = widget.props.checked;
  }

  @override
  Widget build(BuildContext context) {
    return CheckboxListTile(
      value: _checked,
      title: widget.props.label != null ? Text(widget.props.label!) : null,
      onChanged: widget.props.disabled
          ? null
          : (value) {
              setState(() => _checked = value ?? false);
              widget.props.onchange?.call(_checked);
            },
    );
  }
}
