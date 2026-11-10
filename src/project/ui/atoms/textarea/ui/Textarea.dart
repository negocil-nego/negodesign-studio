import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';

class Textarea extends StatefulWidget {
  final TextareaProps props;
  const Textarea(this.props, {super.key});

  @override
  State<Textarea> createState() => _TextareaState();
}

class _TextareaState extends State<Textarea> {
  late final TextareaViewModel _vm;

  @override
  void initState() {
    super.initState();
    _vm = TextareaViewModel(widget.props);
  }

  @override
  void didUpdateWidget(covariant Textarea oldWidget) {
    super.didUpdateWidget(oldWidget);
    _vm.didUpdate(oldWidget.props, widget.props);
  }

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: _vm.controller,
      maxLines: widget.props.rows,
      maxLength: widget.props.maxlength,
      enabled: !widget.props.disabled,
      readOnly: widget.props.readonly,
      decoration: InputDecoration(hintText: widget.props.placeholder),
    );
  }
}
