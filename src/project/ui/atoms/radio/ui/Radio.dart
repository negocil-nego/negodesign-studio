import 'package:flutter/material.dart';
import '../data/types.dart';

class Radio extends StatefulWidget {
  final RadioProps props;
  const Radio(this.props, {super.key});

  @override
  State<Radio> createState() => _RadioState();
}

class _RadioState extends State<Radio> {
  late String? _selected;

  @override
  void initState() {
    super.initState();
    _selected = widget.props.value;
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: widget.props.options.map((option) {
        return RadioListTile<String>(
          title: Text(option.label),
          value: option.value,
          groupValue: _selected,
          onChanged: (option.disabled || widget.props.disabled)
              ? null
              : (value) {
                  setState(() => _selected = value);
                  if (value != null) widget.props.onchange?.call(value);
                },
        );
      }).toList(),
    );
  }
}
