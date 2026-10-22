import 'package:flutter/material.dart';
import '../data/types.dart';

class Textarea extends StatefulWidget {
  final TextareaProps props;
  const Textarea(this.props, {super.key});

  @override
  State<Textarea> createState() => _TextareaState();
}

class _TextareaState extends State<Textarea> {
  late TextEditingController _controller;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController(text: widget.props.value ?? "");
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: _controller,
      maxLines: widget.props.rows,
      maxLength: widget.props.maxlength,
      enabled: !widget.props.disabled,
      readOnly: widget.props.readonly,
      decoration: InputDecoration(
        hintText: widget.props.placeholder,
      ),
    );
  }
}
