import 'package:flutter/widgets.dart';

import '../../../../atoms/text/data/types.dart';

class SectionGridProps {
  final TextProps? titleProps;
  final TextProps? descriptionProps;
  final String? className;
  final List<Widget> children;

  const SectionGridProps({
    this.titleProps,
    this.descriptionProps,
    this.className,
    this.children = const [],
  });
}
