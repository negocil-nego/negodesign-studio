import 'package:flutter/material.dart';
import '../data/types.dart';

class Select extends StatefulWidget {
  final SelectProps props;
  const Select(this.props, {super.key});

  @override
  State<Select> createState() => _SelectState();
}

class _SelectState extends State<Select> {
  late String? _value;

  @override
  void initState() {
    super.initState();
    _value = widget.props.value;
  }

  @override
  Widget build(BuildContext context) {
    return DropdownButtonFormField<String>(
      value: _value,
      hint: widget.props.placeholder != null
          ? Text(widget.props.placeholder!)
          : null,
      items: widget.props.options
          .map((o) => DropdownMenuItem(
                value: o.value,
                enabled: !o.disabled,
                child: Text(o.label),
              ))
          .toList(),
      onChanged: widget.props.disabled ? null : (v) => setState(() => _value = v),
    );
  }
}
