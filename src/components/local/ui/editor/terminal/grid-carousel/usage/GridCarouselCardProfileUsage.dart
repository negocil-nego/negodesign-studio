import 'package:flutter/widgets.dart';
import '../../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid-carousel/data/types.dart';
import '../../../../../../../project/ui/molecules/card/profile/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-title/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-image/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid-carousel/card/profile/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid-carousel/card/profile/ui/GridCarouselCardProfile.dart';

class GridCarouselCardProfileExample extends StatelessWidget {
  const GridCarouselCardProfileExample({super.key});

  @override
  Widget build(BuildContext context) => GridCarouselCardProfile(
        GridCarouselCardProfileProps(
          sectionGridCarouselProps: const SectionGridCarouselProps(
            titleProps: TextProps(Text: 'GridCarouselCardProfile'),
            descriptionProps: TextProps(Text: 'Cartões apresentados com um componente reutilizável.'),
            rows: 2,
            columns: 2,
          ),
          items: const [
            CardProfileProps(
              titleProps: CardTitleProps(title: 'Design system'),
              imageProps: CardImageProps(src: '/img/ux-01.jpg', alt: 'Equipa a trabalhar num design system'),
            ),
            CardProfileProps(
              titleProps: CardTitleProps(title: 'Componentes reutilizáveis'),
              imageProps: CardImageProps(src: '/img/programmer-01.jpg', alt: 'Componentes de software'),
            ),
          ],
        ),
      );
}
