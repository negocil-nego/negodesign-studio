import 'package:flutter/material.dart';

/// Aproximação em Flutter do átomo Popover.
/// O Flutter não expõe o atributo HTML `popover`; como não existe equivalente
/// directo, apresentamos o trigger e o painel empilhados numa Column.
class Popover extends StatelessWidget {
  final String id;
  final Widget trigger;
  final Widget panel;
  final String role;
  final String? ariaLabel;

  const Popover({
    required this.id,
    required this.trigger,
    required this.panel,
    this.role = 'dialog',
    this.ariaLabel,
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return Column(children: [trigger, panel]);
  }
}
