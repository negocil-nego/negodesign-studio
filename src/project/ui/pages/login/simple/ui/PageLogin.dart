import 'package:flutter/material.dart';
import '..\..\..\..\templates\section\simple\ui\SimpleSection.dart';
import '../../../../organisms/section/form/data/types.dart';
import '..\..\..\..\molecules\form\ui\Form.dart';
import '../../../../atoms/text/data/types.dart';
import '../../../../atoms/button/variant/default/type/types.dart';

class PageLogin extends StatelessWidget {
  const PageLogin({super.key});
  @override
  Widget build(BuildContext context) => SimpleSection(sectionFormProps: SectionFormProps(isScreen: true, titleProps: const TextProps(Text: 'Iniciar sessão'), subtitleProps: const TextProps(Text: 'Bem-vindo de volta'), descriptionProps: const TextProps(Text: 'Aceda à sua conta para continuar.'), formProps: const FormConfig(grid: false, fields: [FormFieldConfig(label: 'Email', type: 'email', hint: 'nome@exemplo.com', required: true), FormFieldConfig(label: 'Palavra-passe', type: 'password', hint: 'A sua palavra-passe', required: true)]), buttonSubmitProps: const ButtonProps(text: 'Entrar'), buttonsLoginProps: const [ButtonProps(text: 'Ainda não tem conta? Criar conta', variant: 'outline') ]));
}
