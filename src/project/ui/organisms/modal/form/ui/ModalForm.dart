import 'package:flutter/material.dart';
import '..\..\..\..\molecules\form\ui\Form.dart';
import '../../../../molecules/form/data/types.dart';

class ModalForm extends StatelessWidget {
  final bool open;
  final String title;
  final FormProps? form;
  final VoidCallback? onClose;
  const ModalForm({super.key, this.open = false, this.title = '', this.form, this.onClose});
  @override
  Widget build(BuildContext context) {
    if (open) WidgetsBinding.instance.addPostFrameCallback((_) { if (context.mounted) showDialog<void>(context: context, builder: (_) => AlertDialog(title: Text(title), content: form == null ? null : ProjectForm(form!), actions: [TextButton(onPressed: () { Navigator.pop(context); onClose?.call(); }, child: const Text('Fechar'))])); });
    return const SizedBox.shrink();
  }
}
