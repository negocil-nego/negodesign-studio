import '../../../../../../molecules/card/product/data/types.dart';
import '../../../data/types.dart';

class GridCarouselCardProductProps {
  final SectionGridCarouselProps? sectionGridCarouselProps;
  final List<CardProductProps> items;

  const GridCarouselCardProductProps({
    this.sectionGridCarouselProps,
    required this.items,
  });
}
