import '../../../../../../molecules/card/media/data/types.dart';
import '../../../buttons/data/types.dart';

class CarouselCardMediaProps {
  final SectionCarouselButtonsProps? sectionCarouselButtonsProps;
  final List<CardMediaProps> items;

  const CarouselCardMediaProps({
    this.sectionCarouselButtonsProps,
    required this.items,
  });
}
