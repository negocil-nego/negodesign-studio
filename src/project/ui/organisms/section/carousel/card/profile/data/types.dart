import '../../../../../../molecules/card/profile/data/types.dart';
import '../../../buttons/data/types.dart';

class CarouselCardProfileProps {
  final SectionCarouselButtonsProps? sectionCarouselButtonsProps;
  final List<CardProfileProps> items;

  const CarouselCardProfileProps({
    this.sectionCarouselButtonsProps,
    required this.items,
  });
}
