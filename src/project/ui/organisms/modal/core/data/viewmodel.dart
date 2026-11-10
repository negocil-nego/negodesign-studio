import 'package:flutter/material.dart';

class ModalViewModel {
  bool shown = false;

  void sync({
    required BuildContext context,
    required bool mounted,
    required bool open,
    required Widget child,
    VoidCallback? onClose,
  }) {
    if (!mounted) return;
    if (open && !shown) {
      shown = true;
      showDialog<void>(context: context, builder: (_) => Dialog(child: child)).whenComplete(() {
        shown = false;
        onClose?.call();
      });
    } else if (!open && shown && Navigator.of(context).canPop()) {
      Navigator.of(context).pop();
    }
  }
}

