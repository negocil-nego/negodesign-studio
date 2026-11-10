import 'package:flutter/material.dart';
import '../data/viewmodel.dart';

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
  final ModalViewModel _vm = ModalViewModel();

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _syncModal());
  }

  @override
  void didUpdateWidget(covariant Modal oldWidget) {
    super.didUpdateWidget(oldWidget);
    WidgetsBinding.instance.addPostFrameCallback((_) => _syncModal());
  }

  void _syncModal() {
    _vm.sync(context: context, mounted: mounted, open: widget.open, child: widget.child, onClose: widget.onClose);
  }

  @override
  Widget build(BuildContext context) => const SizedBox.shrink();
}
