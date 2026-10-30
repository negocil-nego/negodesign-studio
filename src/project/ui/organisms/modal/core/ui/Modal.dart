import 'package:flutter/material.dart';

class Modal extends StatefulWidget {
  final bool open;
  final String? title;
  final Widget child;
  final VoidCallback? onClose;
  const Modal({super.key, this.open = false, this.title, required this.child, this.onClose});
  @override
  State<Modal> createState() => _ModalState();
}
class _ModalState extends State<Modal> {
  bool shown = false;
  @override
  void initState() { super.initState(); WidgetsBinding.instance.addPostFrameCallback((_) => _sync()); }
  @override
  void didUpdateWidget(covariant Modal oldWidget) { super.didUpdateWidget(oldWidget); WidgetsBinding.instance.addPostFrameCallback((_) => _sync()); }
  void _sync() { if (!mounted) return; if (widget.open && !shown) { shown = true; showDialog<void>(context: context, builder: (_) => Dialog(child: widget.child)).whenComplete(() { shown = false; widget.onClose?.call(); }); } else if (!widget.open && shown && Navigator.of(context).canPop()) Navigator.of(context).pop(); }
  @override
  Widget build(BuildContext context) => const SizedBox.shrink();
}
