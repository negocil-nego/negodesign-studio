import 'package:flutter/material.dart';
import '..\..\..\..\templates\section\dual-section\ui\DualSection.dart';
import '../../../../organisms/section/form/data/types.dart';
import '..\..\..\..\molecules\form\ui\Form.dart';
import '../../../../atoms/text/data/types.dart';
import '../../../../atoms/button/variant/default/type/types.dart';

class PageLoginDual extends StatelessWidget {
  const PageLoginDual({super.key});
  @override
  Widget build(BuildContext context) => DualSection(leftSectionForm: SectionFormProps(isScreen: true, className: 'bg-blue-600 text-white', titleProps: const TextProps(Text: 'Negodesign'), subtitleProps: const TextProps(Text: 'Crie com confiança.'), descriptionProps: const TextProps(Text: 'Os seus projectos e ferramentas num só lugar.')), rightSectionForm: SectionFormProps(isScreen: true, titleProps: const TextProps(Text: 'Iniciar sessão'), subtitleProps: const TextProps(Text: 'Bem-vindo de volta.'), formProps: const FormConfig(grid: false, fields: [FormFieldConfig(label: 'Email', type: 'email', hint: 'nome@exemplo.com', required: true), FormFieldConfig(label: 'Palavra-passe', type: 'password', hint: 'A sua palavra-passe', required: true)]), buttonSubmitProps: const ButtonProps(text: 'Entrar'), buttonsLoginProps: const [ButtonProps(text: 'Não tem conta? Registe-se', variant: 'outline')]));
}
