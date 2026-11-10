import 'package:flutter/material.dart';
import '../../../../templates/section/dual-section/ui/DualSection.dart';
import '../../../../organisms/section/form/data/types.dart';
import '../../../../molecules/form/ui/Form.dart';
import '../../../../atoms/text/data/types.dart';
import '../../../../atoms/button/variant/default/type/types.dart';

class PageLoginDual extends StatelessWidget {
  const PageLoginDual({super.key});

  @override
  Widget build(BuildContext context) {
    return DualSection(
      leftSectionForm: const SectionFormProps(
        isScreen: true,
        className: 'bg-blue-600 text-white',
        titleProps: TextProps(Text: 'Negodesign'),
        subtitleProps: TextProps(Text: 'Crie com confiança.'),
        descriptionProps: TextProps(
          Text: 'Os seus projectos e ferramentas num só lugar.',
        ),
      ),
      rightSectionForm: const SectionFormProps(
        isScreen: true,
        titleProps: TextProps(Text: 'Iniciar sessão'),
        subtitleProps: TextProps(Text: 'Bem-vindo de volta.'),
        formProps: FormConfig(
          grid: false,
          fields: [
            FormFieldConfig(
              label: 'Email',
              type: 'email',
              hint: 'nome@exemplo.com',
              required: true,
            ),
            FormFieldConfig(
              label: 'Palavra-passe',
              type: 'password',
              hint: 'A sua palavra-passe',
              required: true,
            ),
          ],
        ),
        buttonSubmitProps: ButtonProps(text: 'Entrar'),
        buttonsLoginProps: [
          ButtonProps(
            text: 'Não tem conta? Registe-se',
            variant: 'outline',
          ),
        ],
      ),
    );
  }
}
