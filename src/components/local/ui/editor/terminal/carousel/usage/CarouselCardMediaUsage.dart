import 'package:flutter/widgets.dart';
import '../../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../../project/ui/molecules/card/media/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-image/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-title/data/types.dart';
import '../../../../../../../project/ui/organisms/section/carousel/buttons/data/types.dart';
import '../../../../../../../project/ui/organisms/section/carousel/card/media/data/types.dart';
import '../../../../../../../project/ui/organisms/section/carousel/card/media/ui/CarouselCardMedia.dart';

class ProjectCarouselExample extends StatelessWidget {
  const ProjectCarouselExample({super.key});

  @override
  Widget build(BuildContext context) => CarouselCardMedia(
        CarouselCardMediaProps(
          sectionCarouselButtonsProps: const SectionCarouselButtonsProps(
            titleProps: TextProps(Text: 'Projectos recentes'),
            descriptionProps: TextProps(Text: 'Uma selecção de trabalhos da equipa.'),
          ),
          items: const [
            CardMediaProps(
              titleProps: CardTitleProps(title: 'Design system'),
              imageProps: CardImageProps(src: '/img/ux-01.jpg', alt: 'Equipa a trabalhar num design system'),
            ),
            CardMediaProps(
              titleProps: CardTitleProps(title: 'Componentes reutilizáveis'),
              imageProps: CardImageProps(src: '/img/programmer-01.jpg', alt: 'Componentes de software'),
            ),
          ],
        ),
      );
}
