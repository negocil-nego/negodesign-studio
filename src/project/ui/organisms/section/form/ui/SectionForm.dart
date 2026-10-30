import 'package:flutter/material.dart';
import '../../../../atoms/button/variant/default/ui/Button.dart';
import '../../../../atoms/button/variant/default/type/types.dart';
import '../../../../atoms/text/data/types.dart';
import '../../../../atoms/text/ui/text.dart' as textui;
import '..\..\..\..\molecules\form\ui\Form.dart';

class SectionForm extends StatelessWidget {
  final bool isScreen;
  final String? className;
  final TextProps? titleProps, subtitleProps, descriptionProps;
  final FormConfig? formProps;
  final ButtonProps? buttonSubmitProps;
  final List<ButtonProps> buttonsLoginProps;
  const SectionForm({super.key, this.isScreen = false, this.className, this.titleProps, this.subtitleProps, this.descriptionProps, this.formProps, this.buttonSubmitProps, this.buttonsLoginProps = const []});
  @override
  Widget build(BuildContext context) => SizedBox(width: double.infinity, height: isScreen ? double.infinity : null, child: Center(child: ConstrainedBox(constraints: const BoxConstraints(maxWidth: 448), child: Padding(padding: const EdgeInsets.all(24), child: Column(mainAxisSize: MainAxisSize.min, children: [
    if (titleProps != null) textui.Text(titleProps!), if (subtitleProps != null) textui.Text(subtitleProps!), if (descriptionProps != null) Padding(padding: const EdgeInsets.symmetric(vertical: 12), child: textui.Text(descriptionProps!)),
    if (formProps != null) ProjectForm(formProps!), if (buttonSubmitProps != null) Padding(padding: const EdgeInsets.only(top: 16), child: Button(buttonSubmitProps!)),
    if (buttonsLoginProps.isNotEmpty) Wrap(spacing: 8, children: buttonsLoginProps.map((p) => Button(p)).toList()),
  ])))));
}
