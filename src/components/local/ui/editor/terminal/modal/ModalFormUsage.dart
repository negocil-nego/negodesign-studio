import 'package:flutter/material.dart';
import '../../../../../../project/ui/organisms/modal/form/ui/ModalForm.dart';
import '../../../../../../project/ui/molecules/form/data/types.dart';

class ModalFormUsage extends StatefulWidget {
  const ModalFormUsage({super.key});
  @override
  State<ModalFormUsage> createState() => _ModalFormUsageState();
}

class _ModalFormUsageState extends State<ModalFormUsage> {
  bool open = false;
  final FormProps form = const FormProps(style: 'grid', fields: [
    FormIt(component: 'input-text', id: 'project-name', labelProps: FormLabelProps(label: 'Nome do projecto', forId: 'project-name'), inputProps: FormInputProps(id: 'project-name', name: 'projectName', type: 'text', placeholder: 'O seu projecto')),
    FormIt(component: 'input-email', id: 'contact-email', labelProps: FormLabelProps(label: 'Email de contacto', forId: 'contact-email'), inputProps: FormInputProps(id: 'contact-email', name: 'email', type: 'email', placeholder: 'nome@exemplo.com')),
  ]);
  @override
  Widget build(BuildContext context) => Stack(children: [
    Center(child: ElevatedButton(onPressed: () => setState(() => open = true), child: const Text('Abrir formulário'))),
    ModalForm(open: open, title: 'Novo projecto', form: form, onClose: () => setState(() => open = false)),
  ]);
}
