import '../../../../../../molecules/card/product/data/types.dart';
import '../../../buttons/data/types.dart';

class CarouselCardProductProps {
  final SectionCarouselButtonsProps? sectionCarouselButtonsProps;
  final List<CardProductProps> items;

  const CarouselCardProductProps({
    this.sectionCarouselButtonsProps,
    required this.items,
  });
}
