import 'package:flutter/material.dart';
import '../../../../../../project/ui/atoms/text/data/types.dart';
import '../../../../../../project/ui/organisms/section/carousel/buttons/data/types.dart';
import '../../../../../../project/ui/organisms/section/carousel/buttons/ui/SectionCarouselButtons.dart';

class SectionCarouselButtonsUsage extends StatelessWidget {
  const SectionCarouselButtonsUsage({super.key});

  @override
  Widget build(BuildContext context) {
    const people = [
      ('Blake Star', 'COO, blackstar.com', '/img/programmer-01.jpg', 5),
      ('David Gilmore', 'CTO, darkside.ai', '/img/ux-01.jpg', 5),
      ('Gerard White', 'COO, exo4emo.ai', '/img/programmer-02.jpg', 5),
      ('Azuny', 'COO, studio.example', '/img/ux-01.jpg', 4),
    ];

    return SectionCarouselButtons(
      SectionCarouselButtonsProps(
        titleProps: const TextProps(Text: 'O que dizem os nossos clientes'),
        descriptionProps: const TextProps(Text: 'Histórias de pessoas que usam a plataforma no dia-a-dia.'),
        children: people.map((person) => Card(
          clipBehavior: Clip.antiAlias,
          child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
            Image.asset(person.$3, width: double.infinity, height: 224, fit: BoxFit.cover),
            Padding(padding: const EdgeInsets.all(16), child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              Text('${'★' * person.$4}${'☆' * (5 - person.$4)}', style: const TextStyle(color: Colors.amber)),
              const SizedBox(height: 8),
              Text(person.$1, style: const TextStyle(fontWeight: FontWeight.w600)),
              Text(person.$2, style: const TextStyle(fontSize: 12, color: Colors.grey)),
            ])),
          ]),
        )).toList(),
      ),
    );
  }
}
