import 'package:flutter/material.dart';
import '../../../../molecules/form/ui/Form.dart';
import '../../../../molecules/form/data/types.dart';
import '../../core/ui/Modal.dart';

class ModalForm extends StatelessWidget {
  final bool open;
  final String title;
  final FormProps? form;
  final VoidCallback? onClose;

  const ModalForm({
    super.key,
    this.open = false,
    this.title = '',
    this.form,
    this.onClose,
  });

  @override
  Widget build(BuildContext context) {
    return Modal(
      open: open,
      title: title,
      onClose: onClose,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (title.isNotEmpty)
            Padding(
              padding: const EdgeInsets.only(bottom: 12),
              child: Text(
                title,
                style: const TextStyle(fontSize: 18, fontWeight: FontWeight.w600),
              ),
            ),
          if (form != null) ProjectForm(form!),
        ],
      ),
    );
  }
}
