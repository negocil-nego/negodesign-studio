import 'package:flutter/material.dart';
import '../data/types.dart';
import '../data/viewmodel.dart';

class SectionCarouselButtons extends StatefulWidget {
  final SectionCarouselButtonsProps props;
  const SectionCarouselButtons(this.props, {super.key});

  @override
  State<SectionCarouselButtons> createState() => _SectionCarouselButtonsState();
}

class _SectionCarouselButtonsState extends State<SectionCarouselButtons> {
  final SectionCarouselButtonsViewModel _vm = SectionCarouselButtonsViewModel();

  @override
  void dispose() {
    _vm.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final props = widget.props;
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: const EdgeInsets.only(bottom: 24),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    if (props.titleProps != null)
                      Text(props.titleProps!.Text, style: const TextStyle(fontSize: 28, fontWeight: FontWeight.w600)),
                    if (props.descriptionProps != null)
                      Padding(padding: const EdgeInsets.only(top: 8), child: Text(props.descriptionProps!.Text, style: const TextStyle(color: Colors.grey))),
                  ],
                ),
              ),
              IconButton.filledTonal(onPressed: () => _vm.scroll(-1), tooltip: 'Previous items', icon: const Icon(Icons.arrow_back)),
              const SizedBox(width: 8),
              IconButton.filledTonal(onPressed: () => _vm.scroll(1), tooltip: 'Next items', icon: const Icon(Icons.arrow_forward)),
            ],
          ),
        ),
        SizedBox(
          width: double.infinity,
          height: 360,
          child: LayoutBuilder(
            builder: (context, constraints) => ListView.separated(
              controller: _vm.controller,
              scrollDirection: Axis.horizontal,
              itemCount: props.children.length,
              separatorBuilder: (_, __) => const SizedBox(width: 20),
              itemBuilder: (_, index) => SizedBox(width: _vm.cardWidth(constraints.maxWidth), child: props.children[index]),
            ),
          ),
        ),
      ],
    );
  }
}
