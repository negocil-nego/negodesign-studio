import 'package:flutter/material.dart';
import '../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../project/ui/organisms/section/carousel/image/data/types.dart';
import '../../../../../../project/ui/organisms/section/carousel/image/ui/SectionCarousel.dart';

class SectionCarouselImageUsage extends StatelessWidget {
  const SectionCarouselImageUsage({super.key});

  @override
  Widget build(BuildContext context) => Center(
    child: ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 360),
      child: SectionCarousel(SectionImageCarouselProps(
        backgroundColor: 'bg-gradient-to-t from-blue-950/70 to-transparent',
        items: const [
          SectionImageCarouselItem(backgroundImage: 'assets/programmer-01.jpg', title: TextProps(Text: 'Explore novos lugares'), description: TextProps(Text: 'Descubra experiências e ideias para a sua próxima viagem.')),
          SectionImageCarouselItem(backgroundImage: 'assets/ux-01.jpg', title: TextProps(Text: 'Inspire-se todos os dias'), description: TextProps(Text: 'Encontre histórias que ajudam a transformar os seus projectos.')),
          SectionImageCarouselItem(backgroundImage: 'assets/programmer-02.jpg', title: TextProps(Text: 'Partilhe o seu próximo destino'), description: TextProps(Text: 'Guarde os momentos e partilhe-os com a sua comunidade.')),
        ],
      )),
    ),
  );
}
