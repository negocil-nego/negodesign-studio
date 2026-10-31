import '../../../../../atoms/text/data/types.dart';

class SectionImageCarouselItem {
  final TextProps? title;
  final TextProps? description;
  final String? backgroundImage;

  const SectionImageCarouselItem({this.title, this.description, this.backgroundImage});
}

class SectionImageCarouselProps {
  final List<SectionImageCarouselItem> items;
  final String? className;
  /// CSS hex/gradient values or supported Tailwind background utility classes.
  final String? backgroundColor;

  const SectionImageCarouselProps({this.items = const [], this.className, this.backgroundColor});
}
