import '../../../../atoms/button/variant/default/type/types.dart';
import '../../../../atoms/text/data/types.dart';
import '../../../../molecules/form/ui/Form.dart';

class SectionFormProps {
  final bool isScreen;
  final String? className;
  final TextProps? titleProps;
  final TextProps? subtitleProps;
  final TextProps? descriptionProps;
  final FormConfig? formProps;
  final ButtonProps? buttonSubmitProps;
  final List<ButtonProps> buttonsLoginProps;

  const SectionFormProps({
    this.isScreen = false,
    this.className,
    this.titleProps,
    this.subtitleProps,
    this.descriptionProps,
    this.formProps,
    this.buttonSubmitProps,
    this.buttonsLoginProps = const [],
  });
}
