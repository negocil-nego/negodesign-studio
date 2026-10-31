import 'package:flutter/widgets.dart';
import '../../../../../atoms/text/data/types.dart';

class SectionCarouselButtonsProps {
  final TextProps? titleProps;
  final TextProps? descriptionProps;
  final String? className;
  final List<Widget> children;

  const SectionCarouselButtonsProps({
    this.titleProps,
    this.descriptionProps,
    this.className,
    required this.children,
  });
}
