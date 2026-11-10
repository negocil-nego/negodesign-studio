import 'package:flutter/material.dart';
import '../../../../organisms/section/form/data/types.dart';
import '../../../../organisms/section/form/ui/SectionForm.dart';

class DualSection extends StatelessWidget {
  final SectionFormProps leftSectionForm;
  final SectionFormProps rightSectionForm;

  const DualSection({
    super.key,
    required this.leftSectionForm,
    required this.rightSectionForm,
  });

  Widget _buildSection(SectionFormProps props) {
    return SectionForm(
      isScreen: props.isScreen,
      className: props.className,
      titleProps: props.titleProps,
      subtitleProps: props.subtitleProps,
      descriptionProps: props.descriptionProps,
      formProps: props.formProps,
      buttonSubmitProps: props.buttonSubmitProps,
      buttonsLoginProps: props.buttonsLoginProps,
    );
  }

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        final left = _buildSection(leftSectionForm);
        final right = _buildSection(rightSectionForm);
        if (constraints.maxWidth >= 768) {
          return Row(
            children: [
              Expanded(child: left),
              Expanded(child: right),
            ],
          );
        }
        return Column(children: [left, right]);
      },
    );
  }
}
