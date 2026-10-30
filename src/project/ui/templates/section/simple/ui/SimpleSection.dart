import 'package:flutter/material.dart';
import '../../../../organisms/section/form/data/types.dart';
import '..\..\..\..\organisms\section\form\ui\SectionForm.dart';
class SimpleSection extends StatelessWidget { final SectionFormProps sectionFormProps; const SimpleSection({super.key, required this.sectionFormProps}); @override Widget build(BuildContext context) => SizedBox(height: double.infinity, child: SectionForm(isScreen: sectionFormProps.isScreen, className: sectionFormProps.className, titleProps: sectionFormProps.titleProps, subtitleProps: sectionFormProps.subtitleProps, descriptionProps: sectionFormProps.descriptionProps, formProps: sectionFormProps.formProps, buttonSubmitProps: sectionFormProps.buttonSubmitProps, buttonsLoginProps: sectionFormProps.buttonsLoginProps)); }
