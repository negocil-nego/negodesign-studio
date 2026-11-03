import 'package:flutter/widgets.dart';
import '../../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid/data/types.dart';
import '../../../../../../../project/ui/molecules/card/profile/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-title/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-image/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid/card/profile/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid/card/profile/ui/GridCardProfile.dart';

class GridCardProfileExample extends StatelessWidget {
  const GridCardProfileExample({super.key});

  @override
  Widget build(BuildContext context) => GridCardProfile(
        GridCardProfileProps(
          sectionGridProps: const SectionGridProps(
            titleProps: TextProps(Text: 'GridCardProfile'),
            descriptionProps: TextProps(Text: 'Cartões apresentados com um componente reutilizável.'),
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
