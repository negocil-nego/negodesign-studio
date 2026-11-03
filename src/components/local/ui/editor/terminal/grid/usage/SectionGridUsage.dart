import 'package:flutter/widgets.dart';
import '../../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../../project/ui/molecules/card/media/data/types.dart';
import '../../../../../../../project/ui/molecules/card/product/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-title/data/types.dart';
import '../../../../../../../project/ui/molecules/card/shared/card-image/data/types.dart';
import '../../../../../../../project/ui/molecules/card/media/ui/CardMedia.dart';
import '../../../../../../../project/ui/molecules/card/product/ui/CardProduct.dart';
import '../../../../../../../project/ui/organisms/section/grid/data/types.dart';
import '../../../../../../../project/ui/organisms/section/grid/ui/SectionGrid.dart';

class SectionGridExample extends StatelessWidget {
  const SectionGridExample({super.key});

  @override
  Widget build(BuildContext context) => SectionGrid(
        SectionGridProps(
          titleProps: TextProps(Text: 'SectionGrid'),
          descriptionProps: TextProps(Text: 'Organize cartões numa apresentação responsiva.'),
          children: [
            CardMedia(const CardMediaProps(
              titleProps: CardTitleProps(title: 'Design system'),
              imageProps: CardImageProps(src: '/img/ux-01.jpg', alt: 'Equipa a trabalhar'),
            )),
            CardProduct(const CardProductProps(
              titleProps: CardTitleProps(title: 'Componentes reutilizáveis'),
              imageProps: CardImageProps(src: '/img/programmer-01.jpg', alt: 'Código num monitor'),
            )),
          ],
        ),
      );
}
