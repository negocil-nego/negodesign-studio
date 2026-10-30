import 'package:flutter/material.dart';
import '../../../../../../project/ui/organisms/modal/core/ui/Modal.dart';

class ModalUsage extends StatefulWidget {
  const ModalUsage({super.key});
  @override
  State<ModalUsage> createState() => _ModalUsageState();
}

class _ModalUsageState extends State<ModalUsage> {
  bool open = false;
  @override
  Widget build(BuildContext context) => Stack(children: [
    Center(child: ElevatedButton(onPressed: () => setState(() => open = true), child: const Text('Abrir modal'))),
    Modal(open: open, title: 'Detalhes do projecto', onClose: () => setState(() => open = false), child: Padding(padding: const EdgeInsets.all(20), child: Column(mainAxisSize: MainAxisSize.min, crossAxisAlignment: CrossAxisAlignment.start, children: [const Text('Negodesign', style: TextStyle(color: Colors.blue)), const SizedBox(height: 8), const Text('Detalhes do projecto', style: TextStyle(fontSize: 20, fontWeight: FontWeight.w600)), const SizedBox(height: 8), const Text('Use o modal para apresentar conteúdo contextual sem sair da página actual.'), Align(alignment: Alignment.centerRight, child: TextButton(onPressed: () => setState(() => open = false), child: const Text('Fechar')))]))),
  ]);
}
