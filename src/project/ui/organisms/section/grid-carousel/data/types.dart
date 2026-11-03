import 'package:flutter/widgets.dart';

import '../../../../atoms/text/data/types.dart';

class SectionGridCarouselProps {
  final TextProps? titleProps;
  final TextProps? descriptionProps;
  final String? className;
  final int rows;
  final int columns;
  final List<Widget> children;

  const SectionGridCarouselProps({
    this.titleProps,
    this.descriptionProps,
    this.className,
    this.rows = 1,
    this.columns = 4,
    this.children = const [],
  });
}
