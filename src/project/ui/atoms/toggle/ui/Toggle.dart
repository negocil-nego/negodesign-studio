import 'package:flutter/material.dart';
import '../data/types.dart';

class Toggle extends StatefulWidget {
  final ToggleProps props;
  const Toggle(this.props, {super.key});

  @override
  State<Toggle> createState() => _ToggleState();
}

class _ToggleState extends State<Toggle> {
  late bool _checked;

  @override
  void initState() {
    super.initState();
    _checked = widget.props.checked;
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Switch(
          value: _checked,
          onChanged: widget.props.disabled
              ? null
              : (value) {
                  setState(() => _checked = value);
                  widget.props.onchange?.call(value);
                },
        ),
        if (widget.props.label != null) ...[
          const SizedBox(width: 8),
          Text(widget.props.label!),
        ],
      ],
    );
  }
}
