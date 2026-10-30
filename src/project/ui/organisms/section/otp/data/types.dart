import 'package:flutter/material.dart';
import '../../../../atoms/button/variant/default/type/types.dart';
import '../../../../atoms/text/data/types.dart';
import '../../../../atoms/input/children/otp/data/types.dart';
class SectionOtpProps {
  final bool isScreen; final String layout; final String? className;
  final TextProps? brandTitleProps, brandSubtitleProps, brandDescriptionProps, titleProps, subtitleProps, descriptionProps;
  final ButtonProps? buttonSubmitProps; final List<ButtonProps> buttonsLoginProps; final InputOtpProps? inputOtpProps;
  const SectionOtpProps({this.isScreen=false, this.layout='simple', this.className, this.brandTitleProps, this.brandSubtitleProps, this.brandDescriptionProps, this.titleProps, this.subtitleProps, this.descriptionProps, this.buttonSubmitProps, this.buttonsLoginProps=const [], this.inputOtpProps});
}
